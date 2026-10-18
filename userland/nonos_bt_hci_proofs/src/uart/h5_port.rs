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

//! The simulated controller behind a Three-wire UART port. Its side of the
//! link is a second `H5`; each command that side takes draws the sim's
//! answer, sent back as reliable events as the window allows.

use std::collections::VecDeque;

use crate::sim::config::Config;
use crate::sim::controller::respond;
use nonos_bt_hci::uart::h5::header::{COMMAND, EVENT};
use nonos_bt_hci::uart::h5::H5;

pub struct H5Port {
    pub cfg: Config,
    pub peer: H5,
    pub to_host: VecDeque<u8>,
    pub events: VecDeque<Vec<u8>>,
    pub commands: Vec<Vec<u8>>,
    pub clock: u64,
}

impl H5Port {
    pub fn new(cfg: Config) -> Self {
        let (to_host, events) = (VecDeque::new(), VecDeque::new());
        Self { cfg, peer: H5::default(), to_host, events, commands: Vec::new(), clock: 0 }
    }

    pub(super) fn serve(&mut self) {
        while let Some((kind, p)) = self.peer.inbox.pop_front() {
            if kind == COMMAND {
                self.events.extend(respond(&self.cfg, &p));
                self.commands.push(p);
            }
        }
        while self.events.front().is_some_and(|e| self.peer.send(EVENT, e)) {
            self.events.pop_front();
        }
        while let Some(o) = self.peer.poll_out(self.clock, 10) {
            self.to_host.extend(o);
        }
    }
}
