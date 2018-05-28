extern crate stdweb;

mod event;
mod gamepad;
mod mapping;
mod monitor;

pub use event::{
    Event,
    EventData,
};
pub use gamepad::{
    GamepadDescription,
    GamepadMappingType,
    GamepadState,
    GamepadStateChange,
};
pub use mapping::{
    Axis,
    Button,
    Mapping,
};
pub use monitor::Monitor;
