use ::gamepad::{
    GamepadDescription,
    GamepadStateChange,
};
use ::mapping::{
    Axis,
    Button,
    Mapping,
};

use std::fmt::{
    self,
    Display,
    Formatter
};
use std::rc::Rc;

/// The data associated with an event, not including the gamepad itself.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventData {
    /// A gamepad was connected.
    Connected,
    /// A gamepad was disconnected.
    Disconnected,
    /// An axis was moved. (axis index, value [-1–1])
    Axis(usize, f64),
    /// A button was pressed or released. (button index, pressed)
    Button(usize, bool),
    /// A button's value (the amount it is pressed) changed. (button index, value [0–1])
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

/// An event, including connections, disconnections, and button/axis input.
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    /// A description of the gamepad which emitted this event.
    pub gamepad: Rc<GamepadDescription>,
    /// The event type and values.
    pub data: EventData
}

/// An input event, mapped to one of the standard buttons or axes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MappedEvent {
    /// An axis was moved. [-1–1]
    Axis(Axis, f64),
    /// A button's value (the amount it is pressed) changed. [0–1].
    ButtonValue(Button, f64),
    /// A button was pressed.
    ButtonPress(Button),
    /// A button was released.
    ButtonRelease(Button),
}

impl Event {
    pub(crate) fn new(gamepad: Rc<GamepadDescription>, data: EventData) -> Self {
        Self {
            gamepad,
            data,
        }
    }

    /// Convert this raw event to a standard input event.
    ///
    /// The button-index or axis-index is mapped to a [Button](enum.Button.html) or [Axis](enum.Axis.html).
    /// Returns `None` if the gamepad mapping is unknown,
    /// or the event is a non-input event (i.e. connect or disconnect).
    pub fn map(&self) -> Option<MappedEvent> {
        match self.data {
            EventData::Axis(i, val) =>
                self.gamepad.mapping.map_axis(i).map(|a|
                    MappedEvent::Axis(a, val)
                ),
            EventData::Button(i, true) =>
                self.gamepad.mapping.map_button(i).map(|b|
                    MappedEvent::ButtonPress(b)
                ),
            EventData::Button(i, false) =>
                self.gamepad.mapping.map_button(i).map(|b|
                    MappedEvent::ButtonRelease(b)
                ),
            EventData::ButtonValue(i, val) =>
                self.gamepad.mapping.map_button(i).map(|b|
                    MappedEvent::ButtonValue(b, val)
                ),
            _ => None
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
