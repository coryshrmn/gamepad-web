extern crate gamepad_web;
extern crate stdweb;

use gamepad_web::Monitor;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::web::{
    document,
    window,
};

/// Write a new line to the "log" DOM element.
fn log(msg: &str) {
    let out_elem = document().query_selector("#output").unwrap().unwrap();

    let msg_elem = document().create_element("p").unwrap();
    msg_elem.set_text_content(msg);

    if let Some(child) = out_elem.first_child() {
        out_elem.insert_before(&msg_elem, &child).unwrap();
    }
    else {
        out_elem.append_child(&msg_elem);
    }
}

fn animate(monitor: Rc<RefCell<Monitor>>) {

    while let Some(ev) = monitor.borrow_mut().poll() {
        log(&ev.to_string());
    }

    // queue another animate() on the next frame
    window().request_animation_frame(move |_| animate(monitor));
}

fn main() {
    stdweb::initialize();

    let monitor = Rc::new(RefCell::new(Monitor::new()));

    log("Waiting for gamepad events...");

    animate(monitor);

    stdweb::event_loop();
}
