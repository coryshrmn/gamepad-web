extern crate stdweb;

mod event;
mod gamepad;
mod monitor;

pub use event::{
    AnalogChange,
    DigitalChange,
    Event,
    EventData,
};
pub use gamepad::{
    GamepadDescription,
    GamepadMappingType,
    GamepadState,
    GamepadStateChange,
};
pub use monitor::Monitor;
