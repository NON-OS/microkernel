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

//! The connectors notifications named, read a few per tick and their
//! changes acknowledged; a read that sticks hands the PPM to `recover`.

use super::ppm::Ppm;
use super::session::{Session, State, PER_TICK};
use super::start::read_port;
use alloc::vec::Vec;

impl Session {
    pub(super) fn serve(&mut self, ppm: &mut impl Ppm, now: u64) {
        // Connector numbers the PPM has no port for are dropped, not kept.
        let all = (1u128 << (self.ports.len() + 1)) - 2;
        if self.storm.take_rescan(now) {
            self.pending |= all;
        }
        self.pending &= all;
        let due: Vec<u8> = (1..=self.ports.len() as u8)
            .filter(|n| self.pending & (1u128 << n) != 0)
            .take(PER_TICK)
            .collect();
        for n in due {
            self.pending &= !(1u128 << n);
            match read_port(ppm, &mut self.pending, n, true) {
                Ok(s) => self.update(n, s),
                Err(_) => {
                    self.state = State::Recovering { failures: 0, at: now };
                    return;
                }
            }
        }
    }
}
