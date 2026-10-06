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

//! CCMP replay detection (IEEE Std 802.11-2020, 12.5.3.4.4): a received
//! frame's packet number must exceed the last one accepted under the same key
//! and traffic identifier, or the frame is a replay and is discarded. Counters
//! are kept per TID because the access point numbers frames once per key and
//! frames of different TIDs may arrive out of order; a non-QoS frame counts
//! as TID 0. A group key's counters start at the Key RSC the AP delivered it
//! with. The check runs whether the chip or the station decrypted the frame:
//! a chip that decrypts in hardware does not check packet numbers.

/// TIDs a QoS Control field can name.
const TIDS: usize = 16;
/// Group key indices.
const GROUP_KEYS: usize = 4;

#[derive(Clone, Copy)]
pub(super) struct Replay {
    unicast: [u64; TIDS],
    group: [[u64; TIDS]; GROUP_KEYS],
}

impl Replay {
    pub(super) const fn new() -> Self {
        Self { unicast: [0; TIDS], group: [[0; TIDS]; GROUP_KEYS] }
    }

    /// Start a group key's counters at the Key RSC it came with.
    pub(super) fn set_group(&mut self, key_id: u8, rsc: u64) {
        if let Some(g) = self.group.get_mut(key_id as usize) {
            *g = [rsc; TIDS];
        }
    }

    /// Accept `pn` for a pairwise (`group` = None) or group frame on `tid`,
    /// advancing the counter, or refuse it as a replay.
    pub(super) fn accept(&mut self, group: Option<u8>, tid: u8, pn: u64) -> bool {
        let tid = (tid as usize) % TIDS;
        let slot = match group {
            None => &mut self.unicast[tid],
            Some(k) => match self.group.get_mut(k as usize) {
                Some(g) => &mut g[tid],
                None => return false,
            },
        };
        if pn <= *slot {
            return false;
        }
        *slot = pn;
        true
    }
}
