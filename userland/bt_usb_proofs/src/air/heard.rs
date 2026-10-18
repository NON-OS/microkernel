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

//! What the controller passes on of the peer's advertising. An undirected
//! advertisement always reaches the host. A directed one reaches it as an
//! ADV_DIRECT_IND report when its TargetA is the host's own address; when
//! TargetA is some other resolvable private address, only under the
//! extended filter policy and as an LE Directed Advertising Report (Vol 6,
//! Part B, 4.3.3; Vol 4, Part E, 7.7.65.11). Otherwise it is dropped.

use super::events::advertised;
use super::state::Air;

const ACCEPT_UNRESOLVED: u8 = 0x02;

impl Air {
    pub(super) fn peer_heard(&self) -> Option<Vec<u8>> {
        let Some(target) = self.seen.directed_to else {
            return Some(advertised(self.seen.from, self.adv_kind, &self.adv));
        };
        if Some(target) == self.seen.random {
            return Some(advertised(self.seen.from, 0x01, &[]));
        }
        let rpa = target[5] & 0xc0 == 0x40;
        let policy = self.seen.scan_filters.last() == Some(&ACCEPT_UNRESOLVED);
        (rpa && policy).then(|| directed(self.seen.from, target))
    }
}

/// An LE Directed Advertising Report (7.7.65.11) with one report: from
/// `from`, a random address, aimed at `target`; RSSI -50.
pub fn directed(from: [u8; 6], target: [u8; 6]) -> Vec<u8> {
    let mut sub = vec![0x0b, 1, 0x01, 0x01];
    sub.extend_from_slice(&from);
    sub.push(0x01);
    sub.extend_from_slice(&target);
    sub.push(-50i8 as u8);
    let mut e = vec![0x3e, sub.len() as u8];
    e.extend_from_slice(&sub);
    e
}
