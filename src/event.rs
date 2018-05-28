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
pub enum EventData {
    Connected,
    Disconnected,
    Axis(usize, f64),
    Button(usize, bool),
    ButtonValue(usize, f64),
}

impl<'a> From<&'a GamepadStateChange> for EventData {
    fn from(change: &'a GamepadStateChange) -> Self {
        match change {
            &GamepadStateChange::Axis(index, value) => EventData::Axis(index, value),
            &GamepadStateChange::Button(index, pressed) => EventData::Button(index, pressed),
            &GamepadStateChange::ButtonValue(index, value) => EventData::ButtonValue(index, value),
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
            &EventData::Connected => write!(f, "connected"),
            &EventData::Disconnected => write!(f, "disconnected"),
            &EventData::Axis(index, value) => write!(f, "Axis {}: {:.3}", index, value),
            &EventData::Button(index, pressed) => write!(f, "Button {}: {}", index, if pressed { "pressed" } else { "released" }),
            &EventData::ButtonValue(index, value) => write!(f, "Button {}: {}", index, value),
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Pad {} {}", self.gamepad.index, self.data)?;
        if self.data == EventData::Connected {
            write!(f, " {}", self.gamepad.name)?;
        }
        Ok(())
    }
}
