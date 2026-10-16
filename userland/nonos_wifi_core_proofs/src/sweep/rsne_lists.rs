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
//! The RSNE's lists, and its fields after the AKM suites.

use super::rng::Rng;
use super::rsne_shape::{Field, Sound};
use nonos_wifi_core::rsn::suite::selector;

/// A count and up to three suites of `types` (or any): their selectors.
pub fn list(s: &mut Sound, rng: &mut Rng, types: &[u8], size: usize) -> Vec<u32> {
    let n = rng.below(4);
    s.put(Field::Count { size }, &(n as u16).to_le_bytes());
    let mut out = Vec::new();
    for _ in 0..n {
        let oui = if rng.below(6) == 0 { [rng.byte(), 0, 0] } else { [0, 15, 172] };
        let b =
            [oui[0], oui[1], oui[2], if rng.below(6) == 0 { rng.byte() } else { rng.pick(types) }];
        out.push(selector(b));
        s.put(Field::Entry, &b);
    }
    out
}

/// RSN Capabilities, the PMKID list and the group management cipher, as
/// far as `stop` goes.
pub fn rest(s: &mut Sound, rng: &mut Rng, stop: usize) {
    if stop >= 4 {
        let caps = rng.next() as u16;
        s.want.caps = caps;
        s.put(Field::Fixed, &caps.to_le_bytes());
    }
    if stop >= 5 {
        let n = rng.below(3);
        s.put(Field::Count { size: 16 }, &(n as u16).to_le_bytes());
        for _ in 0..n {
            let id = rng.bytes(16);
            s.put(Field::Entry, &id);
        }
    }
    if stop >= 6 {
        let b = [0, 15, 172, rng.pick(&[6u8, 11, 12, 13])];
        s.want.group_mgmt = Some(selector(b));
        s.put(Field::GroupMgmt, &b);
    }
}
