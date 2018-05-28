extern crate gamepad_web;
extern crate stdweb;

use gamepad_web::{
    Axis,
    Button,
    MappedEvent,
    Monitor,
};

use stdweb::traits::*;
use stdweb::web::{
    document,
    Element,
    window,
};

use std::time::{Duration, Instant};

struct State {
    monitor: Monitor,
    player: Element,
    position: f64,
    velocity: f64,
    jump_start: Option<Instant>,
}

fn deadzone(val: f64, zone: f64) -> f64 {
    let mag = (val.abs() - zone) / (1.0 - zone);
    if mag < 0.0 {
        0.0
    }
    else {
        mag * val.signum()
    }
}

impl State {
    fn new() -> Self {
        Self {
            monitor: Monitor::new(),
            player: document().query_selector("#player").unwrap().unwrap(),
            position: 50.0,
            velocity: 0.0,
            jump_start: None,
        }
    }

    fn update(&mut self) {
        while let Some(event) = self.monitor.poll_mapped() {
            match event {
                MappedEvent::ButtonPress(Button::South) =>
                    self.jump(),
                MappedEvent::ButtonRelease(Button::South) =>
                    self.end_jump(),
                MappedEvent::Axis(Axis::LeftStickX, x) =>
                    self.velocity = deadzone(x, 0.15),
                _ => (),
            }
        }

        self.walk();

        if let Some(time) = self.jump_start {
            if time.elapsed() > Duration::from_millis(500) {
                self.end_jump();
            }
        }
    }

    fn jump(&mut self) {
        self.jump_start = Some(Instant::now());
        self.player.set_attribute("class", "jump").unwrap();
    }

    fn end_jump(&mut self) {
        self.jump_start = None;
        self.player.set_attribute("class", "").unwrap();
    }

    fn walk(&mut self) {
        self.position += self.velocity;
        self.player.set_attribute("style", &format!("left: {}%", self.position)).unwrap();
    }

}

fn advance(mut state: State) {
    state.update();
    window().request_animation_frame(move |_| advance(state));
}

fn main() {
    stdweb::initialize();

    let state = State::new();
    advance(state);

    stdweb::event_loop();
}
