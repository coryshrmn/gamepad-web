# Event-based Gamepad Web Backend

[Documentation](https://coryshrmn.github.io/gamepad-web/doc/gamepad_web/index.html)

This crate extends the
[Gamepad Web API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API),
to emit events for button and axis state changes.

Our goal is to resemble a native API,
which emits an event for each state change,
unlike the Web API,
which only allows state querying.

## Example

```rust
use gamepad_web::*;

fn main() {
    // start listening for Gamepad events
    let mut monitor = Monitor::new();

    // start animation loop
    next_frame(monitor);
}

fn next_frame(mut monitor: Monitor) {
    // check for new input events
    while let Some(event) = monitor.poll_mapped() {
        match event {
            MappedEvent::ButtonPress(Button::South) => // "A" on Xbox
                jump(),
            MappedEvent::Axis(Axis::LeftStickX, x) =>
                set_velocity(x),
            _ => ()
        }
    }

    // queue the next frame
    request_animation_frame(move |_| next_frame(monitor));
}

```

See the full [mapping example](examples/mapping), using [stdweb](https://github.com/koute/stdweb) for `request_animation_frame`.

## Running the Examples

1. Install [cargo-web](https://github.com/koute/cargo-web)

        $ cargo install cargo-web

2. Start an example

        $ cd examples/mapping
        $ cargo web start

3. Browse [localhost:8000](http://localhost:8000).

The compiled examples are also hosted here:

* [log](https://coryshrmn.github.io/gamepad-web/examples/log/deploy/index.html): view raw events
* [layout](https://coryshrmn.github.io/gamepad-web/examples/layout/deploy/index.html): visualize gamepad state, mapped to standard gamepads where available
* [mapping](https://coryshrmn.github.io/gamepad-web/examples/mapping/deploy/index.html): use the simplest API for single-player interaction

## License

Dual licensed under [Apache](LICENSE-APACHE) and [MIT](LICENSE-MIT).
Use Apache unless your lawyers won't let you.
