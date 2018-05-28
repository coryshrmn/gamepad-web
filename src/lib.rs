//! # Event-based Gamepad Web Backend
//!
//! [GitHub repository](https://github.com/coryshrmn/gamepad-web)
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
//! fn main() {
//!     // start listening for Gamepad events
//!     let mut monitor = Monitor::new();
//!
//!     // start animation loop
//!     next_frame(monitor);
//! }
//! fn next_frame(mut monitor: Monitor) {
//!     // process new input events
//!     while let Some(event) = monitor.poll_mapped() {
//!         match event {
//!             MappedEvent::ButtonPress(Button::South) => // "A" on Xbox
//!                 jump(),
//!             MappedEvent::Axis(Axis::LeftStickX, x) =>
//!                 set_velocity(x),
//!             _ => ()
//!         }
//!     }
//!
//!     // queue the next frame
//!     request_animation_frame(move |_| next_frame(monitor));
//! }
//! # fn jump() {}
//! # fn set_velocity(x: f64) {}
//! # fn request_animation_frame(callback: impl FnOnce(f64) + 'static) {}
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
