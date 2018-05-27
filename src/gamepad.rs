use stdweb::web::{
    Gamepad,
    IGamepad,
    IGamepadButton,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Change {
    Axis(usize, f64),
    Button(usize, bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Snapshot {
    pub timestamp: f64,
    pub axes: Vec<f64>,
    pub buttons: Vec<bool>,
}

impl Snapshot {
    pub fn new(timestamp: f64, axes_count: usize, button_count: usize) -> Self {
        Self {
            timestamp,
            axes: vec![0.0; axes_count],
            buttons: vec![false; button_count],
        }
    }

    /// Set all axes/buttons to 0.0/unpressed.
    pub fn clear(&mut self) {
        for a in &mut self.axes {
            *a = 0.0;
        }
        for b in &mut self.buttons {
            *b = false;
        }
    }

    pub fn diff(&self, previous: &Self) -> Vec<Change> {
        let changed_buttons = previous.buttons.iter()
            .zip(self.buttons.iter())
            .enumerate()
            .filter(|(_, (old, new))| old != new)
            .map(|(i, (_, &new))| Change::Button(i, new));

        let changed_axes = previous.axes.iter()
            .zip(self.axes.iter())
            .enumerate()
            .filter(|(_, (old, new))| old.to_bits() != new.to_bits())
            .map(|(i, (_, &new))| Change::Axis(i, new));

        changed_buttons
            .chain(changed_axes)
            .collect()
    }
}

impl From<Gamepad> for Snapshot {
    fn from(pad: Gamepad) -> Self {
        Self {
            timestamp: pad.timestamp(),
            buttons: pad.buttons().iter()
                .map(IGamepadButton::pressed)
                .collect(),
            axes: pad.axes(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{
        Snapshot,
        Change,
    };

    #[test]
    fn test_snapshot_diff() {
        let empty = Snapshot::new(0.0, 2, 2);

        assert_eq!(empty.diff(&empty), vec![]);

        let mut pressed0 = empty.clone();
        pressed0.buttons[0] = true;

        assert_eq!(pressed0.diff(&empty), vec![Change::Button(0, true)]);
        assert_eq!(empty.diff(&pressed0), vec![Change::Button(0, false)]);

        let mut pressed1 = empty.clone();
        pressed1.buttons[1] = true;

        let p1_diff0 = pressed1.diff(&pressed0);
        assert_eq!(p1_diff0.len(), 2);
        assert!(p1_diff0.contains(&Change::Button(0, false)));
        assert!(p1_diff0.contains(&Change::Button(1, true)));

        let p0_diff1 = pressed0.diff(&pressed1);
        assert_eq!(p0_diff1.len(), 2);
        assert!(p0_diff1.contains(&Change::Button(0, true)));
        assert!(p0_diff1.contains(&Change::Button(1, false)));

        let mut moved = empty.clone();
        moved.axes[0] = 0.125;
        moved.axes[1] = -0.5;

        let m_diff_empty = moved.diff(&empty);
        assert_eq!(m_diff_empty.len(), 2);
        assert!(m_diff_empty.contains(&Change::Axis(0, 0.125)));
        assert!(m_diff_empty.contains(&Change::Axis(1, -0.5)));

        let m_diff_p1 = moved.diff(&pressed1);
        assert_eq!(m_diff_p1.len(), 3);
        assert!(m_diff_p1.contains(&Change::Button(1, false)));
        assert!(m_diff_p1.contains(&Change::Axis(0, 0.125)));
        assert!(m_diff_p1.contains(&Change::Axis(1, -0.5)));

        let p1_diff_m = pressed1.diff(&moved);
        assert_eq!(p1_diff_m.len(), 3);
        assert!(p1_diff_m.contains(&Change::Button(1, true)));
        assert!(p1_diff_m.contains(&Change::Axis(0, 0.0)));
        assert!(p1_diff_m.contains(&Change::Axis(1, 0.0)));
    }
}
