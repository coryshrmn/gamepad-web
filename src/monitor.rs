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
    get_gamepads,
    IGamepad,
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

    /// Update our stored snapshot from given Pad state. Enqueue any changes.
    fn update_pad(&mut self, i: usize, raw: &Gamepad) {

        if self.pads[i].is_none() {
            self.connect_pad(i, raw.into());
        }

        // always true, since we connect ^^^
        if let Some(pad) = &mut self.pads[i] {

            // skip update if we already processed this timestamp
            if pad.state.timestamp == raw.timestamp() {
                return;
            }

            let next_state: GamepadState = raw.into();
            {
                let events = next_state.diff(&pad.state)
                    .map(|change|
                        Event::new(pad.info.clone(), (&change).into())
                    );

                self.queue.extend(events);
            }

            pad.state = next_state;
        }
        else {
            unreachable!();
        }
    }

    /// Reset the pad to None and emit a disconnected event.
    /// Does nothing if pad is already disconnected.
    fn disconnect_pad(&mut self, i: usize) {
        if let Some(pad) = self.pads[i].take() {
            self.queue.push_back(Event::new(pad.info, EventData::Disconnected));
        }
    }

    /// Set the pad to Some() and emit a connected event.
    fn connect_pad(&mut self, i: usize, info: GamepadInfo) {
        let pad: ConnectedPad = info.into();
        self.queue.push_back(Event::new(pad.info.clone(), EventData::Connected));
        self.pads[i] = Some(pad);
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

        // get_gamepads() MUST be called each update.
        // Chrome only updates Gamepad state in get_gamepads()
        // (Counter to MDN documentation, which indicates we can save Gamepad references)
        let raw_pads = get_gamepads();

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
