use stdweb::web::Gamepad;
pub use stdweb::web::GamepadMappingType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GamepadStateChange {
    Axis(usize, f64),
    Button(usize, bool),
    ButtonValue(usize, f64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GamepadDescription {
    pub index: i32,
    pub name: String,
    pub mapping: GamepadMappingType,
    pub axis_count: usize,
    pub button_count: usize,
}

impl<'a> From<&'a Gamepad> for GamepadDescription {
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
    timestamp: f64,
    axes: Vec<f64>,
    buttons: Vec<(bool, f64)>,
}

impl GamepadState {

    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    pub fn axis_count(&self) -> usize {
        self.axes.len()
    }

    pub fn button_count(&self) -> usize {
        self.buttons.len()
    }

    pub fn axis(&self, index: usize) -> f64 {
        self.axes[index]
    }

    pub fn button_pressed(&self, index: usize) -> bool {
        self.buttons[index].0
    }

    pub fn button_value(&self, index: usize) -> f64 {
        self.buttons[index].1
    }

    /// Find the changes in this state, compared to a previous state.
    pub fn diff<'a>(&'a self, previous: &'a Self) -> impl Iterator<Item=GamepadStateChange> + 'a {
        let changed_axes = previous.axes.iter()
            .zip(self.axes.iter())
            .enumerate()
            .filter(|(_, (old, new))| old.to_bits() != new.to_bits())
            .map(|(i, (_, &new))| GamepadStateChange::Axis(i, new));

        let changed_buttons = previous.buttons.iter()
            .zip(self.buttons.iter())
            .enumerate()
            .filter(|(_, (old, new))| old.0 != new.0)
            .map(|(i, (_, &new))| GamepadStateChange::Button(i, new.0));

        let changed_button_values = previous.buttons.iter()
            .zip(self.buttons.iter())
            .enumerate()
            .filter(|(_, (old, new))| old.1.to_bits() != new.1.to_bits())
            .map(|(i, (_, &new))| GamepadStateChange::ButtonValue(i, new.1));

        changed_axes
            .chain(changed_buttons)
            .chain(changed_button_values)
    }
}

impl<'a> From<&'a Gamepad> for GamepadState {

    /// Snapshot the current Gamepad state.
    fn from(pad: &'a Gamepad) -> Self {
        Self {
            timestamp: pad.timestamp(),
            axes: pad.axes(),
            buttons: pad.buttons().iter()
                .map(|b| (b.pressed(), b.value()))
                .collect(),
        }
    }
}

impl<'a> From<&'a GamepadDescription> for GamepadState {

    /// Create a default GamepadState (axes at 0.0, buttons not pressed),
    /// with the given number of axes and buttons.
    ///
    /// Timestamp is -1.0, so this will always be "before" a recorded GamepadState.
    fn from(desc: &'a GamepadDescription) -> Self {
        Self {
            timestamp: -1.0,
            axes: vec![0.0; desc.axis_count],
            buttons: vec![(false, 0.0); desc.button_count],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{
        GamepadDescription,
        GamepadMappingType,
        GamepadState,
        GamepadStateChange,
    };

    #[test]
    fn test_gamepad_state_diff() {

        let desc = GamepadDescription {
            index: 0,
            name: "".into(),
            mapping: GamepadMappingType::NoMapping,
            axis_count: 2,
            button_count: 2,
        };

        let empty: GamepadState = (&desc).into();

        assert_eq!(empty.diff(&empty).collect::<Vec<_>>(), vec![]);

        let mut pressed0 = empty.clone();
        pressed0.buttons[0] = true;

        assert_eq!(pressed0.diff(&empty).collect::<Vec<_>>(), vec![GamepadStateChange::Button(0, true)]);
        assert_eq!(empty.diff(&pressed0).collect::<Vec<_>>(), vec![GamepadStateChange::Button(0, false)]);

        let mut pressed1 = empty.clone();
        pressed1.buttons[1] = true;

        let p1_diff0: Vec<_> = pressed1.diff(&pressed0).collect();
        assert_eq!(p1_diff0.len(), 2);
        assert!(p1_diff0.contains(&GamepadStateChange::Button(0, false)));
        assert!(p1_diff0.contains(&GamepadStateChange::Button(1, true)));

        let p0_diff1: Vec<_> = pressed0.diff(&pressed1).collect();
        assert_eq!(p0_diff1.len(), 2);
        assert!(p0_diff1.contains(&GamepadStateChange::Button(0, true)));
        assert!(p0_diff1.contains(&GamepadStateChange::Button(1, false)));

        let mut moved = empty.clone();
        moved.axes[0] = 0.125;
        moved.axes[1] = -0.5;

        let m_diff_empty: Vec<_> = moved.diff(&empty).collect();
        assert_eq!(m_diff_empty.len(), 2);
        assert!(m_diff_empty.contains(&GamepadStateChange::Axis(0, 0.125)));
        assert!(m_diff_empty.contains(&GamepadStateChange::Axis(1, -0.5)));

        let m_diff_p1: Vec<_> = moved.diff(&pressed1).collect();
        assert_eq!(m_diff_p1.len(), 3);
        assert!(m_diff_p1.contains(&GamepadStateChange::Button(1, false)));
        assert!(m_diff_p1.contains(&GamepadStateChange::Axis(0, 0.125)));
        assert!(m_diff_p1.contains(&GamepadStateChange::Axis(1, -0.5)));

        let p1_diff_m: Vec<_> = pressed1.diff(&moved).collect();
        assert_eq!(p1_diff_m.len(), 3);
        assert!(p1_diff_m.contains(&GamepadStateChange::Button(1, true)));
        assert!(p1_diff_m.contains(&GamepadStateChange::Axis(0, 0.0)));
        assert!(p1_diff_m.contains(&GamepadStateChange::Axis(1, 0.0)));
    }
}
