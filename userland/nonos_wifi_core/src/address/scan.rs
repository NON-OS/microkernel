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

//! A scan's address: fresh for each scan that transmits.

use super::bits::local_unicast;
use super::keys::Addresses;

/// Draws a scan address may take before giving up on randomness.
const DRAWS: usize = 4;

impl Addresses {
    /// A fresh address for one scan, drawn by `random`: never the last
    /// scan's, and never `join`, the network the scan is for. `None` when
    /// the randomness gave none such.
    pub fn scan(
        &mut self,
        join: [u8; 6],
        mut random: impl FnMut(&mut [u8]) -> bool,
    ) -> Option<[u8; 6]> {
        for _ in 0..DRAWS {
            let mut a = [0u8; 6];
            if !random(&mut a) {
                return None;
            }
            let a = local_unicast(a);
            if Some(a) != self.last_scan && a != join {
                self.last_scan = Some(a);
                return Some(a);
            }
        }
        None
    }
}
