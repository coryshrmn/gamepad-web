// TODO remove
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate gamepad_web;
extern crate stdweb;

use gamepad_web::{
    Axis,
    Button,
    Event,
    EventData,
    GamepadDescription,
    GamepadMappingType,
    Mapping,
    Monitor,
};

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::web::{
    CloneKind,
    document,
    Element,
    window,
};

fn select(selector: &str) -> Element {
    select_from(&document(), selector)
}

fn select_from(parent: &impl IParentNode, selector: &str) -> Element {
    parent.query_selector(selector)
        .expect(&format!("invalid selector \"{}\"", selector))
        .expect(&format!("no such element \"{}\"", selector))
}

fn gamepad_list() -> Element {
    select("#gamepad-list")
}


struct ControllerView {
    mapping: GamepadMappingType,
    root: Element,
}

impl ControllerView {

    /// Create a new view for this gamepad, as a child of elem.
    pub fn new(desc: &GamepadDescription, elem: &impl INode) -> Self {
        let mapping = desc.mapping;
        let root = ControllerView::make_dom(desc);
        elem.append_child(&root);

        select_from(&root, ".gp-name").set_text_content(&desc.name);

        Self {
            mapping,
            root,
        }
    }

    fn make_dom(desc: &GamepadDescription) -> Element {
        match desc.mapping {
            GamepadMappingType::Standard => ControllerView::make_standard_dom(),
            _ => ControllerView::make_nonstandard_dom(desc),
        }
    }

    fn make_standard_dom() -> Element {
        let elem = select("#standard-template").clone_node(CloneKind::Deep).unwrap();
        elem.remove_attribute("id");
        elem
    }

    fn make_nonstandard_dom(dec: &GamepadDescription) -> Element {
        // create elements for each button and axis
        unimplemented!()
    }

    pub fn handle_event(&self, data: &EventData) {
        match data {
            &EventData::Axis(i, val) => self.set_axis(i, val),
            &EventData::Button(i, pressed) => self.set_button(i, pressed),
            &EventData::ButtonValue(i, val) => self.set_button_value(i, val),
            _ => (),
        }
    }

    fn set_button(&self, index: usize, pressed: bool) {

        let selector = if self.mapping == GamepadMappingType::Standard {
            match self.mapping.map_button(index).unwrap() {
                Button::South => ".gp-a",
                Button::East => ".gp-b",
                Button::West => ".gp-x",
                Button::North => ".gp-y",
                Button::LT1 => ".gp-left-triggers .gp-trigger-1",
                Button::RT1 => ".gp-right-triggers .gp-trigger-1",
                Button::LT2 => ".gp-left-triggers .gp-trigger-2",
                Button::RT2 => ".gp-right-triggers .gp-trigger-2",
                Button::Select => ".gp-select",
                Button::Start => ".gp-start",
                Button::LeftStick => ".gp-stick-left",
                Button::RightStick => ".gp-stick-right",
                Button::Up => ".gp-dpad-up",
                Button::Down => ".gp-dpad-down",
                Button::Left => ".gp-dpad-left",
                Button::Right => ".gp-dpad-right",
                Button::Home => ".gp-home",
            }
        }
        else {
            unimplemented!()
        };


        let style = if pressed { "background-color: orange" } else { "" };
        select_from(&self.root, selector).set_attribute("style", style).unwrap();
    }

    fn set_axis(&self, index: usize, val: f64) {

        if self.mapping == GamepadMappingType::Standard {
            let (selector, is_x) = match self.mapping.map_axis(index).unwrap() {
                Axis::LeftStickX => (".gp-stick-left-x", true),
                Axis::LeftStickY => (".gp-stick-left-y", false),
                Axis::RightStickX => (".gp-stick-right-x", true),
                Axis::RightStickY => (".gp-stick-right-y", false),
            };

            let style = format!("{}: {}%",
                if is_x { "left" } else { "top" },
                (50.0 + val * 50.0)
            );

            select_from(&self.root, selector).set_attribute("style", &style).unwrap();

        }
        else {
            unimplemented!()
        }

    }

    fn set_button_value(&self, index: usize, val: f64) {
        if self.mapping == GamepadMappingType::Standard {
            let selector = match self.mapping.map_button(index).unwrap() {
                Button::LT2 => Some(".gp-left-triggers div div"),
                Button::RT2 => Some(".gp-right-triggers div div"),
                _ => None,
            };

            if let Some(selector) = selector {
                let elem = select_from(&self.root, selector);
                let style = format!("bottom: {}%", val * 100.0);
                elem.set_attribute("style", &style).unwrap();
            }
        }
        else {
            unimplemented!()
        }
    }
}

struct State {
    monitor: Monitor,
    views: Vec<Option<ControllerView>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            monitor: Monitor::new(),
            views: vec![],
        }
    }

    pub fn update(&mut self) {
        while let Some(ev) = self.monitor.poll() {
            self.handle_event(&ev);
        }
    }

    fn handle_event(&mut self, ev: &Event) {
        match ev.data {
            EventData::Connected => {
                log(&ev.to_string());
                self.connect(&ev.gamepad);
            },
            EventData::Disconnected => {
                log(&ev.to_string());
                self.disconnect(ev.gamepad.index as usize);
            },
            _ => self.views[ev.gamepad.index as usize].as_ref().unwrap().handle_event(&ev.data),
        }
    }

    fn connect(&mut self, desc: &GamepadDescription) {
        let index = desc.index as usize;
        while index >= self.views.len() {
            self.views.push(None);
        }

        self.views[index] = Some(ControllerView::new(desc, &gamepad_list()))
    }

    fn disconnect(&mut self, index: usize) {
        unimplemented!()
    }
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

    state_rc.borrow_mut().update();

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
