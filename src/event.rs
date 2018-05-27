use ::gamepad::{
    Change,
    GamepadInfo,
};

use std::fmt::{
    self,
    Display,
    Formatter
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Analog {
    pub index: usize,
    pub value: f64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Digital {
    pub index: usize,
    pub pressed: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EventData {
    Connected,
    Disconnected,
    Axis(Analog),
    Button(Digital),
}

impl<'a> From<&'a Change> for EventData {
    fn from(change: &'a Change) -> Self {
        match change {
            &Change::Axis(index, value) => EventData::Axis(Analog { index, value }),
            &Change::Button(index, pressed) => EventData::Button(Digital { index, pressed }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    pub gamepad: Rc<GamepadInfo>,
    pub data: EventData
}

impl Event {
    pub fn new(gamepad: Rc<GamepadInfo>, data: EventData) -> Self {
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
