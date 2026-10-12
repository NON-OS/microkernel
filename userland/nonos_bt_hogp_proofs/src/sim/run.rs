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

//! The host under test and the simulated peripheral, PDU by PDU.

use super::db::Db;
use super::server::Peripheral;
use nonos_bt_hogp::host::{Host, Outcome, Step};
use nonos_bt_hogp::report::Input;
use std::collections::VecDeque;

/// Discovery of a large database takes a few dozen PDUs.
const MAX_PDUS: usize = 2000;

pub struct Run {
    pub host: Host,
    pub peer: Peripheral,
    pub now: u64,
    pub sent: Vec<Vec<u8>>,
    pub input: Vec<Input>,
    pub outcomes: Vec<Outcome>,
    to_peer: VecDeque<Vec<u8>>,
}

impl Run {
    pub fn new(db: Db) -> Self {
        Self::with(Peripheral::new(db))
    }

    pub fn with(peer: Peripheral) -> Self {
        let (host, first) = Host::start(0);
        let mut run = Self {
            host,
            peer,
            now: 0,
            sent: Vec::new(),
            input: Vec::new(),
            outcomes: Vec::new(),
            to_peer: VecDeque::new(),
        };
        run.take(first);
        run
    }

    pub(super) fn take(&mut self, step: Step) {
        self.sent.extend(step.send.iter().cloned());
        self.to_peer.extend(step.send);
        self.input.extend(step.input);
        self.outcomes.extend(step.outcome);
    }

    /// Runs until nothing is in flight; the last outcome.
    pub fn go(&mut self) -> Option<Outcome> {
        for _ in 0..MAX_PDUS {
            let Some(pdu) = self.to_peer.pop_front() else { return self.outcomes.last().copied() };
            if let Some(answer) = self.peer.on(&pdu) {
                self.deliver(&answer);
            }
        }
        panic!("discovery did not settle within {MAX_PDUS} PDUs");
    }
}
