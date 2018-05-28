//! # Event-based Gamepad Web Backend
//!
//! [Documentation](https://coryshrmn.github.io/gamepad-web/doc/gamepad_web/)
//!
//! This crate extends the
//! [Gamepad Web API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API),
//! to emit events for button and axis state changes.
//!
//! Our goal is to resemble a native API,
//! which emits an event for each state change,
//! unlike the Web API,
//! which only allows state querying.
//!
//! ## Quick Start
//!
//! ```
//! use gamepad_web::*;
//!
//! // start listening for Gamepad events
//! let mut monitor = Monitor::new();
//!
//! fn update() {
//!     // process new input events
//!     while let Some(event) = monitor.poll_mapped() {
//!         match event {
//!             MappedEvent::ButtonPress(Button::South) => // "A" on Xbox
//!                 player.jump(),
//!             MappedEvent::Axis(Axis::LeftStickX, x) =>
//!                 player.set_velocity(x),
//!             _ => ()
//!         }
//!     }
//! }
//!
//! ```

#![deny(missing_docs)]

extern crate stdweb;

mod event;
mod gamepad;
mod mapping;
mod monitor;

pub use event::{
    Event,
    EventData,
    MappedEvent,
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
