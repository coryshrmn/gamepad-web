use ::gamepad::{
    GamepadDescription,
    GamepadStateChange,
};

use std::fmt::{
    self,
    Display,
    Formatter
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AnalogChange {
    pub index: usize,
    pub value: f64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DigitalChange {
    pub index: usize,
    pub pressed: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventData {
    Connected,
    Disconnected,
    Axis(AnalogChange),
    Button(DigitalChange),
}

impl<'a> From<&'a GamepadStateChange> for EventData {
    fn from(change: &'a GamepadStateChange) -> Self {
        match change {
            &GamepadStateChange::Axis(index, value) => EventData::Axis(AnalogChange { index, value }),
            &GamepadStateChange::Button(index, pressed) => EventData::Button(DigitalChange { index, pressed }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    pub gamepad: Rc<GamepadDescription>,
    pub data: EventData
}

impl Event {
    pub fn new(gamepad: Rc<GamepadDescription>, data: EventData) -> Self {
        Self {
            gamepad,
            data,
        }
    }
}

impl Display for EventData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            EventData::Connected => write!(f, "connected"),
            EventData::Disconnected => write!(f, "disconnected"),
            EventData::Axis(data) => write!(f, "Axis {}: {:.3}", data.index, data.value),
            EventData::Button(data) => write!(f, "Button {}: {}", data.index, if data.pressed { "pressed" } else { "released" }),
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Pad {} {}", self.gamepad.index, self.data)
    }
}
