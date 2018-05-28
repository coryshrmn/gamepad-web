use ::gamepad::GamepadMappingType;

/// A named button on the standard gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Button {
    /// "A" on Xbox, "cross" on PlayStation, "B" on Nintendo.
    South,
    /// "B" on Xbox, "circle" on PlayStation, "A" on Nintendo.
    East,
    /// "X" on Xbox, "square" on PlayStation, "Y" on Nintendo.
    West,
    /// "Y" on Xbox, "triangle" on PlayStation, "X" on Nintendo.
    North,
    /// "LB" on Xbox, "L1" on PlayStation, "L" on Nintendo.
    LT1,
    /// "RB" on Xbox, "R1" on PlayStation, "R" on Nintendo.
    RT1,
    /// "LT" on Xbox, "L2" on PlayStation, "ZL" on Nintendo.
    LT2,
    /// "RT" on Xbox, "R2" on PlayStation, "ZR" on Nintendo.
    RT2,
    /// "Select" / "Back" / "View" / "Share"
    Select,
    /// "Start" / "Forward" / "Menu" / "Options"
    Start,
    /// Press the left stick, "L3" on PlayStation.
    LeftStick,
    /// Press the right stick, "R3" on PlayStation.
    RightStick,
    /// Up on the directional pad.
    Up,
    /// Down on the directional pad.
    Down,
    /// Left on the directional pad.
    Left,
    /// Right on the directional pad.
    Right,
    /// Home, "Xbox", or "PS", centered on the gamepad.
    Home,
}

/// A named axis on the standard gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    /// From left (-1.0) to right (1.0).
    LeftStickX,
    /// From up (-1.0) to down (1.0).
    LeftStickY,
    /// From left (-1.0) to right (1.0).
    RightStickX,
    /// From up (-1.0) to down (1.0).
    RightStickY,
}

/// A relation between indices and names, for buttons and axes.
pub trait Mapping {
    /// Get the name (if known) of the button at this index.
    fn map_button(&self, index: usize) -> Option<Button>;
    /// Get the index (if known) of this button.
    fn button_index(&self, button: Button) -> Option<usize>;
    /// Get the name (if known) of the axis at this index.
    fn map_axis(&self, index: usize) -> Option<Axis>;
    /// Get the index (if known) of this axis.
    fn axis_index(&self, axis: Axis) -> Option<usize>;
}

impl Mapping for GamepadMappingType {
    fn map_button(&self, index: usize) -> Option<Button> {
        match self {
            GamepadMappingType::Standard =>
                match index {
                    0 => Some(Button::South),
                    1 => Some(Button::East),
                    2 => Some(Button::West),
                    3 => Some(Button::North),
                    4 => Some(Button::LT1),
                    5 => Some(Button::RT1),
                    6 => Some(Button::LT2),
                    7 => Some(Button::RT2),
                    8 => Some(Button::Select),
                    9 => Some(Button::Start),
                    10 => Some(Button::LeftStick),
                    11 => Some(Button::RightStick),
                    12 => Some(Button::Up),
                    13 => Some(Button::Down),
                    14 => Some(Button::Left),
                    15 => Some(Button::Right),
                    16 => Some(Button::Home),
                    _ => None,
                },
            _ => None,
        }
    }

    fn button_index(&self, button: Button) -> Option<usize> {
        #![allow(unreachable_patterns)]
        match self {
            GamepadMappingType::Standard =>
                match button {
                    Button::South => Some(0),
                    Button::East => Some(1),
                    Button::West => Some(2),
                    Button::North => Some(3),
                    Button::LT1 => Some(4),
                    Button::RT1 => Some(5),
                    Button::LT2 => Some(6),
                    Button::RT2 => Some(7),
                    Button::Select => Some(8),
                    Button::Start => Some(9),
                    Button::LeftStick => Some(10),
                    Button::RightStick => Some(11),
                    Button::Up => Some(12),
                    Button::Down => Some(13),
                    Button::Left => Some(14),
                    Button::Right => Some(15),
                    Button::Home => Some(16),
                    _ => None,
                },
            _ => None,
        }
    }

    fn map_axis(&self, index: usize) -> Option<Axis> {
        match self {
            GamepadMappingType::Standard =>
                match index {
                    0 => Some(Axis::LeftStickX),
                    1 => Some(Axis::LeftStickY),
                    2 => Some(Axis::RightStickX),
                    3 => Some(Axis::RightStickY),
                    _ => None,
                },
            _ => None,
        }
    }

    fn axis_index(&self, axis: Axis) -> Option<usize> {
        #![allow(unreachable_patterns)]
        match self {
            GamepadMappingType::Standard =>
                match axis {
                    Axis::LeftStickX => Some(0),
                    Axis::LeftStickY => Some(1),
                    Axis::RightStickX => Some(2),
                    Axis::RightStickY => Some(3),
                    _ => None,
                },
            _ => None,
        }
    }
}
