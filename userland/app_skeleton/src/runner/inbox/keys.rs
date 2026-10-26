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
//! The WM's word that this window took or lost the keyboard: its notice
//! envelope (capsule_wm protocol/notify.rs, pinned in abi/wire.toml as
//! [wm.notify_v1]) of kind 4, naming this window. Who sent it is checked by
//! the caller: only the WM's own pid is believed.

const MAGIC: u32 = 0x4E57_4D56;
const VERSION: u16 = 1;
const LEN: usize = 28;
const KEYS: u32 = 4;

/// True when `msg` says `window_id` has the keyboard now, false when it
/// lost it, None when `msg` is not that notice for this window.
pub fn keys_for(msg: &[u8], window_id: u32) -> Option<bool> {
    let word = |at: usize| u32::from_le_bytes([msg[at], msg[at + 1], msg[at + 2], msg[at + 3]]);
    if msg.len() != LEN || word(0) != MAGIC || u16::from_le_bytes([msg[4], msg[5]]) != VERSION {
        return None;
    }
    (word(8) == KEYS && word(16) == window_id && word(20) <= 1).then(|| word(20) == 1)
}
