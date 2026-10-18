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

//! BRIGHTNESS_GET: each panel's brightness now, read only, for Settings
//! (through the shell). The answer is a count u32, then per panel its
//! identity u32 and its level now in percent u32 (255 when unknown); a
//! fixed length, so a refusal carries the same zeros beside its status.

use super::brightness_rule::Panel;

pub const MAX_SHOWN: usize = 3;
pub const BRIGHTNESS_GET_LEN: usize = 4 + MAX_SHOWN * 8;

/// The first three panels `list` holds.
pub fn encode(list: &[Panel]) -> [u8; BRIGHTNESS_GET_LEN] {
    let mut out = [0u8; BRIGHTNESS_GET_LEN];
    let shown = &list[..list.len().min(MAX_SHOWN)];
    out[..4].copy_from_slice(&(shown.len() as u32).to_le_bytes());
    for (p, e) in shown.iter().zip(out[4..].chunks_exact_mut(8)) {
        e[..4].copy_from_slice(&p.identity.to_le_bytes());
        e[4..].copy_from_slice(&u32::from(p.now).to_le_bytes());
    }
    out
}
