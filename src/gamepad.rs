use ::event::Event;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Drop;
use std::rc::Rc;

use stdweb::web::{
    EventListenerHandle,
    IEventTarget,
    window
};

use stdweb::web::event::{
    ConcreteEvent,
    GamepadConnectedEvent,
    GamepadDisconnectedEvent,
};

pub struct Monitor {
    listener_handles: Vec<EventListenerHandle>,
    queue: Rc<RefCell<VecDeque<Event>>>,
}

impl Monitor {
    pub fn new() -> Self {
        let mut monitor = Self {
            listener_handles: vec![],
            queue: Rc::new(RefCell::new(VecDeque::new())),
        };

        // listen for Web events
        monitor.register_listener::<GamepadConnectedEvent>();
        monitor.register_listener::<GamepadDisconnectedEvent>();

        monitor
    }

    fn register_listener<T: ConcreteEvent + Into<Event>>(&mut self) {
        let queue = self.queue.clone();
        let handle = window().add_event_listener( move |e: T| {
            queue.borrow_mut().push_back(e.into());
        });
        self.listener_handles.push(handle);
    }

    pub fn poll(&self) -> Option<Event> {

        if let Some(ev) = self.queue.borrow_mut().pop_front() {
            return Some(ev)
        }

        None
    }
}

impl Drop for Monitor {
    fn drop(&mut self) {
        while let Some(handle) = self.listener_handles.pop() {
            handle.remove();
        }
    }
}
