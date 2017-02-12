use p25::trunking::fields::TalkGroup;
use std::sync::mpsc::{Sender, Receiver};

use recv::ReceiverEvent;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HubEvent {
    UpdateTalkgroup(TalkGroup),
    UpdateSignalPower(f32),
    UpdateCurFreq(u32),
}

struct AppState {
    pub talkgroup: TalkGroup,
    pub freq: u32,
    pub signal: f32,
}

pub struct MainApp {
    state: AppState,
    events: Receiver<HubEvent>,
    recv: Sender<ReceiverEvent>,
}

impl MainApp {
    pub fn new(events: Receiver<HubEvent>, recv: Sender<ReceiverEvent>) -> Self {
        MainApp {
            state: AppState {
                talkgroup: TalkGroup::Nobody,
                freq: 0,
                signal: 0.0,
            },
            events: events,
            recv: recv,
        }
    }

    pub fn run(&mut self) {
        loop {
            let event = self.events.recv().expect("unable to receive UI event");
            self.handle(event);
        }
    }

    fn handle(&mut self, event: HubEvent) {
        match event {
            HubEvent::UpdateTalkgroup(tg) => self.state.talkgroup = tg,
            HubEvent::UpdateSignalPower(p) => self.state.signal = p,
            HubEvent::UpdateCurFreq(freq) =>  self.state.freq = freq,
        }
    }
}
