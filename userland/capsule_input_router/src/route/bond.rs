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
//! A Bluetooth device's pairing, as the router reads it off an event.
//! The Bluetooth capsule stamps the two high bits of an event's flags (the
//! modifier bits live in the low byte): SOURCE_BT on every event it posts,
//! BOND_AUTHENTICATED when the device's bond was made authenticated (a
//! passkey, or a number the user compared). A device paired by Just Works can
//! be impersonated by anyone in radio range, so its events are kept off the
//! secure path. A Bluetooth event without the level counts as
//! unauthenticated; an event without SOURCE_BT comes from a wired keyboard,
//! mouse or pad and is routed as it always was.

use super::preempt::Route;
use crate::state::key_targets::Key;

pub const SOURCE_BT: u16 = 1 << 15;
pub const BOND_AUTHENTICATED: u16 = 1 << 14;
/// The bits no receiver is handed: the pairing is the router's business.
pub const BOND_BITS: u16 = SOURCE_BT | BOND_AUTHENTICATED;
/// How long after telling the shell it is not told again, so a user typing
/// on such a keyboard at the lock screen sees the notice once, not a stream.
pub const NOTICE_QUIET_MS: i64 = 60_000;

/// What the router keeps for it: when the shell was last told, and the
/// secure attention press it refused (by source and code), whose release
/// goes nowhere either.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bond {
    pub told_ms: Option<i64>,
    pub refused: Option<Key>,
}

/// Whether an event comes from a Bluetooth device whose bond is not
/// authenticated.
pub fn unproven(flags: u16) -> bool {
    flags & SOURCE_BT != 0 && flags & BOND_AUTHENTICATED == 0
}

/// Whether such an event is on the secure path: the secure attention chord,
/// or anything while the trusted band is held (the lock screen, a system
/// prompt) except a release going back to the window its press went to.
pub fn refuse(attention: bool, hold: Option<u32>, route: Route, shell: u32) -> bool {
    if attention {
        return true;
    }
    let Some(holder) = hold else { return false };
    !matches!(route, Route::To(p) if p != holder && p != shell)
}

/// Whether the shell is told of a refusal at `now`.
pub fn due(told_ms: Option<i64>, now: i64) -> bool {
    told_ms.is_none_or(|t| now.saturating_sub(t) >= NOTICE_QUIET_MS)
}
