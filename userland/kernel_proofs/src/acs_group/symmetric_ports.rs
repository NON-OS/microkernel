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

//! What a generated machine's root port holds (symmetric_topologies.rs): a
//! switch with two downstream ports, a conventional PCI bridge with its bus,
//! or a multi-function device.

use super::symmetric_rng::Rng;
use super::symmetric_topologies::Machine;

/// Root port `port` with its buses from `bus`; the last bus it takes.
pub(super) fn below_port(m: &mut Machine, r: &mut Rng, port: u8, bus: u8) -> u8 {
    let kind = r.below(3);
    let last = bus + [3, 1, 0][kind as usize];
    m.push(r, (0, 0x1c, port), Some((bus, last)), false, false);
    match kind {
        0 => {
            m.push(r, (bus, 0, 0), Some((bus + 1, last)), false, false);
            for k in 0..2u8 {
                let below = bus + 2 + k;
                m.push(r, (bus + 1, k, 0), Some((below, below)), false, false);
                m.push(r, (below, 0, 0), None, false, true);
            }
        }
        1 => {
            m.push(r, (bus, 0, 0), Some((bus + 1, bus + 1)), true, false);
            for d in 0..(2 + r.below(2)) as u8 {
                m.push(r, (bus + 1, d, 0), None, false, true);
            }
        }
        _ => {
            for f in 0..(1 + r.below(3)) as u8 {
                m.push(r, (bus, 0, f), None, false, true);
            }
        }
    }
    last
}
