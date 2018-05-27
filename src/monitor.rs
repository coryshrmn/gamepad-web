use ::event::{
    Event,
    EventData,
};
use ::gamepad::{
    GamepadInfo,
    GamepadState,
};

use std::collections::VecDeque;
use std::iter;
use std::rc::Rc;

use stdweb::web::{
    Gamepad,
};

#[derive(Debug, PartialEq, Clone)]
struct ConnectedPad {
    info: Rc<GamepadInfo>,
    state: GamepadState,
}

impl From<GamepadInfo> for ConnectedPad {
    fn from(info: GamepadInfo) -> Self {
        let info = Rc::new(info);
        Self {
            info: info.clone(),
            state: info.as_ref().into(),
        }
    }
}

pub struct Monitor {
    queue: VecDeque<Event>,
    pads: Vec<Option<ConnectedPad>>
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            pads: vec![],
        }
    }

    fn update_state(pad: &mut ConnectedPad, raw: &Gamepad, queue: &mut VecDeque<Event>) {

        // skip update if we already processed this timestamp
        if pad.state.timestamp == raw.timestamp() {
            return;
        }

        let next_state: GamepadState = raw.into();

        // queue any changes as events
        queue.extend( next_state.diff(&pad.state)
            .map(|change| Event::new(pad.info.clone(), (&change).into()))
        );

        pad.state = next_state;
    }

    /// Update our stored snapshot from given Pad state. Enqueue any changes.
    fn update_pad(&mut self, i: usize, raw: &Gamepad) {

        let queue = &mut self.queue;

        let pad = self.pads[i].get_or_insert_with(|| Monitor::make_connected(&raw, queue));
        Monitor::update_state(pad, raw, queue);
    }

    /// Reset the pad to None and emit a disconnected event.
    /// Does nothing if pad is already disconnected.
    fn disconnect_pad(&mut self, i: usize) {
        if let Some(pad) = self.pads[i].take() {
            self.queue.push_back(Event::new(pad.info, EventData::Disconnected));
        }
    }

    /// Creates a ConnectedPad and adds a connected event to queue
    fn make_connected(raw: &Gamepad, queue: &mut VecDeque<Event>) -> ConnectedPad {
        let info: GamepadInfo = raw.into();
        let pad: ConnectedPad = info.into();
        queue.push_back(Event::new(pad.info.clone(), EventData::Connected));
        pad
    }

    fn resize_pads(&mut self, size: usize) {

        let orig_size = self.pads.len();

        // grow to handle more gamepads than before
        let extra_pads = iter::repeat(None).take(size - orig_size);
        self.pads.extend(extra_pads);

        // shrink to handle fewer gamepads than before
        for i in size..orig_size {
            self.disconnect_pad(i);
        }
        self.pads.truncate(size);
    }

    fn fetch_update(&mut self) {

        // navigator.getGamepads() MUST be called each update.
        // Chrome only updates Gamepad state in get_gamepads()
        // (Counter to MDN documentation, which indicates we can save Gamepad references)
        let raw_pads = Gamepad::get_all();

        self.resize_pads(raw_pads.len());

        // update snapshots for each pad, enqueing any changes
        for (i, raw) in raw_pads.iter().enumerate() {
            match raw {
                None => self.disconnect_pad(i),
                Some(raw) => self.update_pad(i, &raw),
            }
        }
    }

    pub fn poll(&mut self) -> Option<Event> {

        if self.queue.is_empty() {
            self.fetch_update();
        }

        self.queue.pop_front()
    }
}
