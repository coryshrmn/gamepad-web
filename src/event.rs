use stdweb::web::{
    Gamepad,
    IGamepad,
};

use stdweb::web::event::{
    GamepadConnectedEvent,
    GamepadDisconnectedEvent,
    IGamepadEvent,
};

use std::fmt::{
    self,
    Display,
    Formatter
};

// TODO test mock Gamepad

#[derive(Debug, PartialEq, Clone)]
pub struct AnalogEvent {
    pub pad: Gamepad,
    pub index: usize,
    pub value: f64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DigitalEvent {
    pub pad: Gamepad,
    pub index: usize,
    pub pressed: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Connected(Gamepad),
    Disconnected(Gamepad),
    Axis(AnalogEvent),
    Button(DigitalEvent),
}

impl From<GamepadConnectedEvent> for Event {
    fn from(ev: GamepadConnectedEvent) -> Self {
        Event::Connected(ev.gamepad())
    }
}

impl From<GamepadDisconnectedEvent> for Event {
    fn from(ev: GamepadDisconnectedEvent) -> Self {
        Event::Disconnected(ev.gamepad())
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Event::Connected(pad) => write!(f, "Pad {} connected", pad.index()),
            Event::Disconnected(pad) => write!(f, "Pad {} disconnected", pad.index()),
            Event::Axis(data) => write!(f, "Pad {} Axis {}: {:.3}", data.pad.index(), data.index, data.value),
            Event::Button(data) => write!(f, "Pad {} Button {}: {}", data.pad.index(), data.index, if data.pressed { "pressed" } else { "released" }),
        }
    }
}
