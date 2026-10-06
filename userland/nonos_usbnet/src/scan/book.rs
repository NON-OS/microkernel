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

//! Which root ports to try, and which to leave. Pure, for the proofs.
//!
//! A port is tried until it binds, holds another kind of device, or fails
//! `TRIES` times. A device replugged, or one that came back as another (a
//! phone switching USB tethering on), is tried afresh: its port was seen
//! empty, or the controller reports its connection changed.

use alloc::vec::Vec;

use super::port::Port;

pub const TRIES: u8 = 3;
const NOT_OURS: u8 = u8::MAX;

pub struct Book {
    tries: [u8; 256],
    present: [bool; 256],
}

impl Book {
    /// The free ports of `ports` to try this pass; ports not connected are
    /// forgotten.
    pub fn plan(&mut self, ports: &[Port]) -> Vec<u8> {
        let mut now = [false; 256];
        for p in ports {
            now[p.id as usize] = true;
            if p.changed || !self.present[p.id as usize] {
                self.tries[p.id as usize] = 0;
            }
        }
        for (tries, _) in self.tries.iter_mut().zip(now).filter(|(_, here)| !here) {
            *tries = 0;
        }
        self.present = now;
        let open = |p: &&Port| p.owner == 0 && self.tries[p.id as usize] < TRIES;
        ports.iter().filter(open).map(|p| p.id).collect()
    }

    pub fn failed(&mut self, port: u8) {
        self.tries[port as usize] = self.tries[port as usize].saturating_add(1).min(TRIES);
    }

    pub fn not_ours(&mut self, port: u8) {
        self.tries[port as usize] = NOT_OURS;
    }

    /// Every port decided against is tried again: a phone whose
    /// re-enumeration the change bit missed is still found.
    pub fn retry_all(&mut self) {
        self.tries = [0; 256];
    }
}

impl Default for Book {
    fn default() -> Self {
        Self { tries: [0; 256], present: [false; 256] }
    }
}
