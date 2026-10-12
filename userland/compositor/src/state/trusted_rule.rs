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

//! The trusted band: the top of the stack, where the lock screen and every
//! permission prompt (capture, pairing, install) are drawn. Only a trusted
//! source may place a layer there, so no client surface can cover or rise
//! above a prompt; SCREENSHOT composes without it, so no client can read a
//! prompt back; and while a layer is up there the input router gives its
//! owner every key and click (`server/trusted_notice.rs`), so a window can
//! neither type into a prompt's place nor click through it. Raising stays
//! inside a layer's own band (`scene_raise`), so nothing is lifted into it.

use super::scene::Layer;

/// Every z at or above this is the trusted band's; the boot splash
/// (4_000_000) and the shell's chrome (3_000_000) sit below it.
pub const TRUSTED_Z: u32 = 0x4000_0000;

/// The services whose process may draw there: the desktop shell draws the
/// lock screen and the prompts.
pub const TRUSTED_SOURCES: [&[u8]; 1] = [b"desktop_shell"];

/// How often a hold is told again (the router lets one lapse after three
/// seconds unheard), and how soon a notice that did not go is tried again.
pub const REFRESH_MS: i64 = 1000;
pub const RETRY_MS: i64 = 100;

pub fn reserved(z: u32) -> bool {
    z >= TRUSTED_Z
}

/// Whether `sender` may place a layer at `z`, given the trusted sources'
/// pids now (None for one not running).
pub fn may_submit(sender: u32, z: u32, trusted: &[Option<u32>]) -> bool {
    !reserved(z) || (sender != 0 && trusted.contains(&Some(sender)))
}

/// The owner of the topmost layer in the trusted band, if one is up.
pub fn holder<'a>(layers: impl IntoIterator<Item = &'a Layer>) -> Option<u32> {
    let top = layers.into_iter().filter(|l| reserved(l.z)).max_by_key(|l| (l.z, l.stack));
    top.map(|l| l.owner_pid)
}

/// Whether the router must be told now: the holder changed since it was
/// last told (tried again every RETRY_MS until a notice goes), or a hold is
/// up and REFRESH_MS passed.
pub fn due(holder: Option<u32>, told: Option<u32>, last_ms: i64, now_ms: i64) -> bool {
    let since = now_ms.saturating_sub(last_ms);
    (holder != told && since >= RETRY_MS) || (holder.is_some() && since >= REFRESH_MS)
}
