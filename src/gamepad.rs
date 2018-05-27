use stdweb::web::{
    Gamepad,
    GamepadButton,
    GamepadMappingType,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Change {
    Axis(usize, f64),
    Button(usize, bool),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GamepadInfo {
    pub index: i32,
    pub name: String,
    pub mapping: GamepadMappingType,
    pub axis_count: usize,
    pub button_count: usize,
}

impl<'a> From<&'a Gamepad> for GamepadInfo {
    fn from(pad: &'a Gamepad) -> Self {
        Self {
            index: pad.index(),
            name: pad.id().into(),
            mapping: pad.mapping(),
            axis_count: pad.axes().len(),
            button_count: pad.buttons().len(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GamepadState {
    pub timestamp: f64,
    pub axes: Vec<f64>,
    pub buttons: Vec<bool>,
}

impl GamepadState {

    /// Find the changes in this state, compared to a previous state.
    pub fn diff<'a>(&'a self, previous: &'a Self) -> impl Iterator<Item=Change> + 'a {
        let changed_axes = previous.axes.iter()
            .zip(self.axes.iter())
            .enumerate()
            .filter(|(_, (old, new))| old.to_bits() != new.to_bits())
            .map(|(i, (_, &new))| Change::Axis(i, new));

        let changed_buttons = previous.buttons.iter()
            .zip(self.buttons.iter())
            .enumerate()
            .filter(|(_, (old, new))| old != new)
            .map(|(i, (_, &new))| Change::Button(i, new));

        changed_axes
            .chain(changed_buttons)
    }
}

impl<'a> From<&'a Gamepad> for GamepadState {

    /// Snapshot the current Gamepad state.
    fn from(pad: &'a Gamepad) -> Self {
        Self {
            timestamp: pad.timestamp(),
            axes: pad.axes(),
            buttons: pad.buttons().iter()
                .map(GamepadButton::pressed)
                .collect(),
        }
    }
}

impl<'a> From<&'a GamepadInfo> for GamepadState {

    /// Create a default GamepadState (axes at 0.0, buttons not pressed),
    /// with the given number of axes and buttons.
    ///
    /// Timestamp is -1.0, so this will always be "before" a recorded GamepadState.
    fn from(info: &'a GamepadInfo) -> Self {
        Self {
            timestamp: -1.0,
            axes: vec![0.0; info.axis_count],
            buttons: vec![false; info.button_count],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{
        Change,
        GamepadInfo,
        GamepadState,
    };

    #[test]
    fn test_gamepad_state_diff() {

        let info = GamepadInfo {
            index: 0,
            name: "".into(),
            mapping: GamepadMappingType::NoMapping,
            axis_count: 2,
            button_count: 2,
        };

        let empty: GamepadState = info.into();

        assert_eq!(empty.diff(&empty).collect::<Vec<_>>(), vec![]);

        let mut pressed0 = empty.clone();
        pressed0.buttons[0] = true;

        assert_eq!(pressed0.diff(&empty).collect::<Vec<_>>(), vec![Change::Button(0, true)]);
        assert_eq!(empty.diff(&pressed0).collect::<Vec<_>>(), vec![Change::Button(0, false)]);

        let mut pressed1 = empty.clone();
        pressed1.buttons[1] = true;

        let p1_diff0: Vec<_> = pressed1.diff(&pressed0).collect();
        assert_eq!(p1_diff0.len(), 2);
        assert!(p1_diff0.contains(&Change::Button(0, false)));
        assert!(p1_diff0.contains(&Change::Button(1, true)));

        let p0_diff1: Vec<_> = pressed0.diff(&pressed1).collect();
        assert_eq!(p0_diff1.len(), 2);
        assert!(p0_diff1.contains(&Change::Button(0, true)));
        assert!(p0_diff1.contains(&Change::Button(1, false)));

        let mut moved = empty.clone();
        moved.axes[0] = 0.125;
        moved.axes[1] = -0.5;

        let m_diff_empty: Vec<_> = moved.diff(&empty).collect();
        assert_eq!(m_diff_empty.len(), 2);
        assert!(m_diff_empty.contains(&Change::Axis(0, 0.125)));
        assert!(m_diff_empty.contains(&Change::Axis(1, -0.5)));

        let m_diff_p1: Vec<_> = moved.diff(&pressed1).collect();
        assert_eq!(m_diff_p1.len(), 3);
        assert!(m_diff_p1.contains(&Change::Button(1, false)));
        assert!(m_diff_p1.contains(&Change::Axis(0, 0.125)));
        assert!(m_diff_p1.contains(&Change::Axis(1, -0.5)));

        let p1_diff_m: Vec<_> = pressed1.diff(&moved).collect();
        assert_eq!(p1_diff_m.len(), 3);
        assert!(p1_diff_m.contains(&Change::Button(1, true)));
        assert!(p1_diff_m.contains(&Change::Axis(0, 0.0)));
        assert!(p1_diff_m.contains(&Change::Axis(1, 0.0)));
    }
}
