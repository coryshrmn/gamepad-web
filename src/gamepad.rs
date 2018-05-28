use stdweb::web::Gamepad;
pub use stdweb::web::GamepadMappingType;

/// A change in axis or button state.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GamepadStateChange {
    /// An axis was moved. (axis index, value [-1–1])
    Axis(usize, f64),
    /// A button was pressed or released. (button index, pressed).
    Button(usize, bool),
    /// A button's value (the amount it is pressed) changed. (button index, value [0–1])
    ButtonValue(usize, f64),
}

/// Information about a gamepad. Remains constant while the gamepad is connected.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GamepadDescription {
    /// The index at which this gamepad connected.
    ///
    /// Increases from 0 as new gamepads are connected,
    /// though an index may be reused after its gamepad disconnects.
    pub index: i32,

    /// Some identifier for the controller.
    ///
    /// Its format depends on the browser and system drivers.
    pub name: String,

    /// The button/axis mapping layout for this gamepad.
    pub mapping: GamepadMappingType,

    /// The number of axes this gamepad reports.
    pub axis_count: usize,

    /// The number of buttons this gamepad reports.
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

/// Snapshot of a gamepad's state at one instant.
#[derive(Debug, PartialEq, Clone)]
pub struct GamepadState {
    timestamp: f64,
    axes: Vec<f64>,
    buttons: Vec<(bool, f64)>,
}

impl GamepadState {

    /// The [DOMHighRestTimeStamp](https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp)
    /// when this gamepad was last updated.
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    /// The number of axes this gamepad reports.
    pub fn axis_count(&self) -> usize {
        self.axes.len()
    }

    /// The number of buttons this gamepad reports.
    pub fn button_count(&self) -> usize {
        self.buttons.len()
    }

    /// Get the value of an axis [-1–1].
    ///
    /// # Panics
    ///
    /// Panics if `index` >= [`axis_count()`](#method.axis_count).
    pub fn axis(&self, index: usize) -> f64 {
        self.axes[index]
    }

    /// Is the button at this index pressed / held down?
    ///
    /// # Panics
    ///
    /// Panics if `index` >= [`button_count()`](#method.button_count).
    pub fn button_pressed(&self, index: usize) -> bool {
        self.buttons[index].0
    }

    /// How much is the button at this index pressed / held down?
    ///
    /// For most buttons on most devices, this is either 0.0 (not pressed) or 1.0 (pressed).
    /// The main exceptions are triggers, which smoothly transition from 0.0 to 1.0 when pressed.
    ///
    /// # Panics
    ///
    /// Panics if `index` >= [`button_count()`](#method.button_count).
    pub fn button_value(&self, index: usize) -> f64 {
        self.buttons[index].1
    }

    /// Find the changes in this state, compared to a previous state.
    ///
    /// The timestamps are not considered.
    /// `previous` could be newer than `self`, this still returns changes from `previous` to `self`.
    pub fn changes_since<'a>(&'a self, previous: &'a Self) -> impl Iterator<Item=GamepadStateChange> + 'a {
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

    /// Snapshot the current gamepad state.
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
    fn test_gamepad_state_changes_since() {

        let desc = GamepadDescription {
            index: 0,
            name: "".into(),
            mapping: GamepadMappingType::NoMapping,
            axis_count: 2,
            button_count: 2,
        };

        let empty: GamepadState = (&desc).into();

        assert_eq!(empty.changes_since(&empty).collect::<Vec<_>>(), vec![]);

        let mut pressed0 = empty.clone();
        pressed0.buttons[0].0 = true;

        assert_eq!(pressed0.changes_since(&empty).collect::<Vec<_>>(), vec![GamepadStateChange::Button(0, true)]);
        assert_eq!(empty.changes_since(&pressed0).collect::<Vec<_>>(), vec![GamepadStateChange::Button(0, false)]);

        let mut pressed1 = empty.clone();
        pressed1.buttons[1].0 = true;

        let p1_changes_since0: Vec<_> = pressed1.changes_since(&pressed0).collect();
        assert_eq!(p1_changes_since0.len(), 2);
        assert!(p1_changes_since0.contains(&GamepadStateChange::Button(0, false)));
        assert!(p1_changes_since0.contains(&GamepadStateChange::Button(1, true)));

        let p0_changes_since1: Vec<_> = pressed0.changes_since(&pressed1).collect();
        assert_eq!(p0_changes_since1.len(), 2);
        assert!(p0_changes_since1.contains(&GamepadStateChange::Button(0, true)));
        assert!(p0_changes_since1.contains(&GamepadStateChange::Button(1, false)));

        let mut moved = empty.clone();
        moved.axes[0] = 0.125;
        moved.axes[1] = -0.5;

        let m_changes_since_empty: Vec<_> = moved.changes_since(&empty).collect();
        assert_eq!(m_changes_since_empty.len(), 2);
        assert!(m_changes_since_empty.contains(&GamepadStateChange::Axis(0, 0.125)));
        assert!(m_changes_since_empty.contains(&GamepadStateChange::Axis(1, -0.5)));

        let m_changes_since_p1: Vec<_> = moved.changes_since(&pressed1).collect();
        assert_eq!(m_changes_since_p1.len(), 3);
        assert!(m_changes_since_p1.contains(&GamepadStateChange::Button(1, false)));
        assert!(m_changes_since_p1.contains(&GamepadStateChange::Axis(0, 0.125)));
        assert!(m_changes_since_p1.contains(&GamepadStateChange::Axis(1, -0.5)));

        let p1_changes_since_m: Vec<_> = pressed1.changes_since(&moved).collect();
        assert_eq!(p1_changes_since_m.len(), 3);
        assert!(p1_changes_since_m.contains(&GamepadStateChange::Button(1, true)));
        assert!(p1_changes_since_m.contains(&GamepadStateChange::Axis(0, 0.0)));
        assert!(p1_changes_since_m.contains(&GamepadStateChange::Axis(1, 0.0)));
    }
}
