# Event-based Gamepad Web Backend

This crate extends the
[Gamepad Web API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API),
to emit events for button and axis state changes.

Our goal is to resemble a native API,
which emits events for each state change,
unlike the Web API,
which only allows state querying.

We use
[stdweb](https://github.com/koute/stdweb)
to interface with the Gamepad Web API.

## Testing

1. Install [cargo-web](https://github.com/koute/cargo-web)

        $ cargo install cargo-web

2. Run the tests

        $ cargo web test --features web_test

## License

Dual licensed under [Apache](LICENSE-APACHE) and [MIT](LICENSE-MIT).
Use Apache unless your lawyers won't let you.
