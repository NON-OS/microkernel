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

//! The compositor's trusted band, as the router keeps it. While the shell
//! has a lock screen or a permission prompt up there, the compositor tells
//! the router so (NIRS op 0x0005, about once a second), and the router gives
//! the prompt's owner every key and every click (`preempt`): no window types
//! into a prompt's place or clicks through it. Only the compositor's pid is
//! believed, and a hold not heard again for HOLD_LEASE_MS lapses, so a
//! compositor that stopped cannot leave every window deaf.

pub const HOLD_LEASE_MS: i64 = 3000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hold {
    pub pid: u32,
    pub until_ms: i64,
}

/// Whether a notice from `sender` is the compositor's own.
pub fn accept(sender: u32, compositor: Option<u32>) -> bool {
    sender != 0 && compositor == Some(sender)
}

/// The hold a notice naming `pid` sets now; 0 ends it.
pub fn told(pid: u32, now_ms: i64) -> Option<Hold> {
    (pid != 0).then_some(Hold { pid, until_ms: now_ms.saturating_add(HOLD_LEASE_MS) })
}

/// Who holds the band now, if the hold has not lapsed.
pub fn holder(hold: Option<Hold>, now_ms: i64) -> Option<u32> {
    hold.filter(|h| now_ms < h.until_ms).map(|h| h.pid)
}
