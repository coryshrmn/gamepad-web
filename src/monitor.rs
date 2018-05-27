use ::event::{
    AnalogEvent,
    DigitalEvent,
    Event,
};
use ::snapshot::{
    Snapshot,
    Change,
};

use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Drop;
use std::rc::Rc;

use stdweb::web::{
    EventListenerHandle,
    Gamepad,
    get_gamepads,
    IEventTarget,
    IGamepad,
    window,
};

use stdweb::web::event::{
    ConcreteEvent,
    GamepadConnectedEvent,
    GamepadDisconnectedEvent,
};

pub struct Monitor {
    listener_handles: Vec<EventListenerHandle>,
    queue: Rc<RefCell<VecDeque<Event>>>,
    snapshots: Vec<Option<Snapshot>>,
}

impl Monitor {
    pub fn new() -> Self {
        let mut monitor = Self {
            listener_handles: vec![],
            queue: Rc::new(RefCell::new(VecDeque::new())),
            snapshots: vec![],
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

    /// Update our stored snapshot from given Pad state. Enqueue any Changes.
    fn update_snap(&mut self, i: usize, pad: &Gamepad) {

        // skip update if we already processed this timestamp
        if let Some(ref snap) = self.snapshots[i] {
            if snap.timestamp == pad.timestamp() {
                return;
            }
        }

        let curr: Snapshot = pad.into();

        let prev = self.snapshots[i].take().unwrap_or_else(|| {
            // blank snap with same button and axes counts
            let mut snap = curr.clone();
            snap.clear();
            snap
        });

        {
            let events = curr.diff(&prev)
                .map(|change| match change {
                    Change::Axis(index, value) => Event::Axis(AnalogEvent { pad: pad.clone(), index, value }),
                    Change::Button(index, pressed) => Event::Button(DigitalEvent { pad: pad.clone(), index, pressed }),
                });

            self.queue.borrow_mut().extend(events);
        }

        self.snapshots[i] = Some(curr);
    }


    fn fetch_update(&mut self) {

        // get_gamepads() NEEDS to be called each update
        // Chrome only updates Gamepad state in get_gamepads().
        // (Counter to MDN documentation, which indicates we can save Gamepad references)
        let pads = get_gamepads();

        // handle more gamepads than before
        while self.snapshots.len() < pads.len() {
            self.snapshots.push(None);
        }

        // handle fewer gamepads than before
        self.snapshots.truncate(pads.len());

        // update snapshots for each pad, enqueing any changes
        for (i, pad) in pads.iter().enumerate() {
            match pad {
                None => self.snapshots[i] = None,
                Some(pad) => self.update_snap(i, &pad),
            }
        }
    }

    pub fn poll(&mut self) -> Option<Event> {

        if self.queue.borrow().is_empty() {
            self.fetch_update();
        }

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
