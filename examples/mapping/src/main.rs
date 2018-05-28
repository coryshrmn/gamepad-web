extern crate gamepad_web;
extern crate stdweb;

use gamepad_web::{
    EventData,
    Monitor,
};

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::web::{
    document,
    Element,
    window,
};

struct ControllerView {
    buttons: Vec<Element>,
    axes: Vec<Element>,
    lt_axis: Element,
    rt_axis: Element,
}

impl ControllerView {
    fn new() -> ControllerView {
        let buttons = [
            ".gp-a",
            ".gp-b",
            ".gp-x",
            ".gp-y",
            ".gp-left-triggers .gp-trigger-1",
            ".gp-right-triggers .gp-trigger-1",
            ".gp-left-triggers .gp-trigger-2",
            ".gp-right-triggers .gp-trigger-2",
            ".gp-select",
            ".gp-start",
            ".gp-stick-left",
            ".gp-stick-right",
            ".gp-dpad-up",
            ".gp-dpad-down",
            ".gp-dpad-left",
            ".gp-dpad-right",
            ".gp-home",
        ];

        let axes = [
            ".gp-stick-left-x",
            ".gp-stick-left-y",
            ".gp-stick-right-x",
            ".gp-stick-right-y",
        ];

        let buttons = buttons.iter().map(|s| select(s));
        let axes = axes.iter().map(|s| select(s));
        let lt_axis = select(".gp-left-triggers div div");
        let rt_axis = select(".gp-right-triggers div div");

        Self {
            buttons: buttons.collect(),
            axes: axes.collect(),
            lt_axis,
            rt_axis,
        }
    }

    fn set_button(&self, index: usize, pressed: bool) {
        if index >= self.buttons.len() {
            return;
        }

        let style = if pressed { "background-color: orange" } else { "" };
        self.buttons[index].set_attribute("style", style).unwrap();
    }

    fn set_axis(&self, index: usize, val: f64) {
        if index >= self.axes.len() {
            return;
        }

        let style = format!("{}: {}%",
            if index % 2 == 0 { "left" } else { "top" },
            (50.0 + val * 50.0)
        );
        self.axes[index].set_attribute("style", &style).unwrap();
    }

    fn set_button_value(&self, index: usize, val: f64) {
        let elem = match index {
            6 => Some(&self.lt_axis),
            7 => Some(&self.rt_axis),
            _ => None,
        };

        if let Some(elem) = elem {
            elem.set_attribute("style", &format!("bottom: {}%", val * 100.0)).unwrap();
        }
    }
}

struct State {
    monitor: Monitor,
    view: ControllerView,
}

impl State {
    fn new() -> Self {
        Self {
            monitor: Monitor::new(),
            view: ControllerView::new(),
        }
    }
}

fn select(selector: &str) -> Element {
    document().query_selector(selector)
        .expect(&format!("invalid selector \"{}\"", selector))
        .expect(&format!("no such element \"{}\"", selector))
}

/// Write a new line to the "log" DOM element.
fn log(msg: &str) {
    let out_elem = select("#output");

    let msg_elem = document().create_element("p").unwrap();
    msg_elem.set_text_content(msg);

    if let Some(child) = out_elem.first_child() {
        out_elem.insert_before(&msg_elem, &child).unwrap();
    }
    else {
        out_elem.append_child(&msg_elem);
    }
}

fn animate(state_rc: Rc<RefCell<State>>) {

    {
        let mut state = state_rc.borrow_mut();

        while let Some(ev) = state.monitor.poll() {
            match ev.data {
                EventData::Connected | EventData::Disconnected => log(&ev.to_string()),
                EventData::Button(i, pressed) => state.view.set_button(i, pressed),
                EventData::Axis(i, val) => state.view.set_axis(i, val),
                EventData::ButtonValue(i, val) => state.view.set_button_value(i, val),
            }
        }
    }

    // queue another animate() on the next frame
    window().request_animation_frame(move |_| animate(state_rc));
}

fn main() {
    stdweb::initialize();

    let state = Rc::new(RefCell::new(State::new()));

    log("Waiting for gamepad events...");

    animate(state);

    stdweb::event_loop();
}
