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

//! A manager over exits of chosen policies, for the choice of circuit by port
//! and the second open. exit_circuit.rs and exit_far.rs play the circuits.

use std::vec::Vec;

use crate::circuit::Hop;
use crate::directory::microdesc::{policy::exits_web, summary::summary};
use crate::manager::{link_tick, open_stream, Bootstrap, Manager, SendError};
use crate::path::Ports;

pub const T0: u64 = super::world::VALID_AFTER + 120;

pub struct Net {
    pub state: Manager,
    pub now: u64,
    pub(super) far: Vec<(u32, Hop)>,
}

/// A Ready manager whose relays are exits with the `p` lines `lines`, at
/// 10.i.0.1 for the i-th, with its link up to one of them and no circuit.
pub fn net(lines: &[&[u8]]) -> Net {
    nonos_libc::seed(0x5EED);
    nonos_libc::set_ms(1_000_000);
    let mut state = Manager::new(1);
    let mut ports = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let mut relay = super::world::relay(i as u32).relay;
        relay.flags.exit = true;
        relay.exits_web = exits_web(line);
        ports.push((relay.rsa_identity, summary(line).unwrap_or_default()));
        state.relays.push(relay);
    }
    state.exits.ports = Ports::new(ports);
    state.bootstrap = Bootstrap::Ready;
    state.valid_until = T0 + 3 * 3600;
    link_tick(&mut state, T0);
    Net { state, now: T0, far: Vec::new() }
}

impl Net {
    /// Open a stream to `port` as caller 7, an install or not, or the refusal.
    pub fn open(&mut self, port: u16, install: bool) -> Result<u16, &'static str> {
        open_stream(&mut self.state, b"node.example", port, self.now, 7, install).map_err(|e| {
            if e == SendError::NoCircuit {
                "no circuit"
            } else {
                "other"
            }
        })
    }
}
