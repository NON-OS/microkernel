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

//! What the hostile sink's DPCD reads give: its capabilities as the run
//! drew them, and a link status moving as the run's profile says.

use super::profile::Status;
use super::sink::Hostile;

impl Hostile {
    /// The bytes a native read from `addr` gets.
    pub(super) fn answer(&mut self, addr: u32, out: &mut [u8]) {
        self.status_reads += (addr == 0x202) as u32;
        for (i, b) in out.iter_mut().enumerate() {
            let a = addr + i as u32;
            *b = match a {
                0..=15 => self.dpcd[a as usize],
                0x202..=0x207 => self.status_byte(a),
                _ => self.rng.byte(),
            };
        }
    }

    fn status_byte(&mut self, a: u32) -> u8 {
        let n = self.status_reads;
        // Each lane's nibble: CR done, EQ done, symbol locked; 0x204 bit 0
        // interlane alignment; 0x206-0x207 the swing each lane asks.
        let asks = |swing: u32| (swing.min(3) as u8) * 0x11;
        match (self.p.status, a) {
            (Status::Settles(k), 0x202 | 0x203) if n > k => 0x77,
            (Status::Settles(k), 0x204) if n > k => 1,
            (Status::Settles(_) | Status::Never, 0x206 | 0x207) => asks(n),
            (Status::Flips, 0x206 | 0x207) => asks(n % 2),
            (Status::Noise, _) => self.rng.byte(),
            _ => 0,
        }
    }
}
