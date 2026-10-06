// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! The simulated Anyone network and a net.anon manager joined to it.

mod blind;
mod descriptor;
mod keys;
mod net;
mod ntor;
mod ring;
mod service;

use std::vec::Vec;

use crate::manager::{link_tick, onion_tick, pump_tick, sendme_tick, Bootstrap, Manager};
use crate::path::Weights;

pub use descriptor::{client_secret, Faults};
pub use net::{relay, SimRelay, World};
pub use service::Service;

/// A fixed throwaway 32 bytes named by `label`.
pub fn keys_bytes32(label: &str) -> [u8; 32] {
    keys::bytes32(label, 0)
}

/// 2026-09-18 19:00 UTC: past the day's time period start, so a fetch uses
/// the current shared random value.
pub const VALID_AFTER: u64 = 1_789_758_000;
const CONSENSUS_RELAYS: u32 = 40;
const INTRO_POINTS: u32 = 3;
const SRV_CURRENT: [u8; 32] = [0x11; 32];
const SRV_PREVIOUS: [u8; 32] = [0x22; 32];

/// The period at `VALID_AFTER`, computed here rather than by the client.
pub fn period() -> u64 {
    (VALID_AFTER / 60 - 12 * 60) / 1440
}

pub struct Net {
    pub state: Manager,
    pub world: World,
    pub now: u64,
    ms: i64,
}

/// A network of forty relays, three introduction points outside the
/// consensus, and a service whose descriptor the responsible HSDirs hold.
/// `shape` may change the relays before the descriptor is placed.
pub fn net(faults: Faults, shape: impl FnOnce(&mut Vec<SimRelay>)) -> Net {
    nonos_libc::seed(0x0A0B_0C0D);
    let mut relays: Vec<SimRelay> = (0..CONSENSUS_RELAYS).map(relay).collect();
    for i in 0..INTRO_POINTS {
        let mut point = relay(100 + i);
        point.relay.flags.hsdir = false;
        point.intro_index = Some(i as usize);
        relays.push(point);
    }
    /* Spare introduction points the service can move to (World::rotate). */
    for i in 0..INTRO_POINTS {
        let mut spare = relay(200 + i);
        spare.relay.flags.hsdir = false;
        spare.listed = false;
        relays.push(spare);
    }
    let service = Service::new(period(), INTRO_POINTS as usize);
    let ids: Vec<[u8; 32]> = relays[..CONSENSUS_RELAYS as usize].iter().map(|r| r.relay.ed25519_identity).collect();
    for at in ring::responsible(&ids, &SRV_CURRENT, &service.blinded.public, period()) {
        relays[at].holds = true;
    }
    shape(&mut relays);

    let mut state = Manager::new(1);
    state.bootstrap = Bootstrap::Ready;
    state.relays = relays[..CONSENSUS_RELAYS as usize].iter().map(|r| r.relay.clone()).collect();
    state.weights = Weights::default();
    state.valid_after = VALID_AFTER;
    state.fresh_until = VALID_AFTER + 3600;
    state.valid_until = VALID_AFTER + 3 * 3600;
    state.srv_current = Some(SRV_CURRENT);
    state.srv_previous = Some(SRV_PREVIOUS);
    let now = VALID_AFTER + 120;
    let ms = 1_000_000;
    nonos_libc::set_ms(ms);
    link_tick(&mut state, now);
    assert!(state.link.is_some(), "the link to the guard is up");
    let world = World::new(relays, service, faults);
    Net { state, world, now, ms }
}

impl Net {
    /// Run the lookup's tick, let the network answer, and pump every reply,
    /// `rounds` times.
    pub fn run(&mut self, rounds: usize) {
        for _ in 0..rounds {
            onion_tick(&mut self.state, self.now);
            self.exchange();
        }
    }

    /// Let the network answer what was sent, and pump all of it in.
    pub fn exchange(&mut self) {
        for _ in 0..64 {
            let link = self.state.link.as_mut().expect("the link stays up");
            if link.sent.is_empty() && link.inbound.is_empty() {
                return;
            }
            self.world.serve(link);
            while !self.state.link.as_ref().unwrap().inbound.is_empty() {
                pump_tick(&mut self.state, self.now);
            }
            sendme_tick(&mut self.state);
        }
    }

    /// Let `seconds` pass on both clocks.
    pub fn pass(&mut self, seconds: u64) {
        self.now += seconds;
        self.ms += seconds as i64 * 1000;
        nonos_libc::set_ms(self.ms);
    }

    pub fn address(&self) -> std::string::String {
        self.world.service.address()
    }

    /// The stream's stage, named, and what has arrived on it; "gone" when
    /// the stream is not in the table.
    pub fn stream(&self, id: u16) -> (std::string::String, Vec<u8>) {
        use crate::stream::StreamStage;
        match self.state.streams.iter().find(|s| s.id == id) {
            None => ("gone".into(), Vec::new()),
            Some(s) => {
                let stage = match s.stage {
                    StreamStage::Opening => "opening".into(),
                    StreamStage::Open => "open".into(),
                    StreamStage::Ended(reason) => std::format!("ended {reason}"),
                };
                (stage, s.inbound.clone())
            }
        }
    }

    /// Open a stream as caller 7, the stream id or the refusal named.
    pub fn open(&mut self, host: &[u8], port: u16) -> Result<u16, &'static str> {
        use crate::manager::SendError;
        crate::manager::open_stream(&mut self.state, host, port, self.now, 7).map_err(|e| match e {
            SendError::BadOnion => "bad onion",
            SendError::TableFull => "table full",
            SendError::NoCircuit => "no circuit",
            SendError::NamesPending => "names pending",
            SendError::NameUnknown => "name unknown",
            SendError::NameChanged => "name changed",
            _ => "other",
        })
    }

    pub fn log(&self) -> Vec<std::string::String> {
        nonos_libc::take_log()
    }
}
