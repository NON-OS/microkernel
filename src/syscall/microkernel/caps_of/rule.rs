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
//! `MkCapsOf`'s answer, pure (kernel_proofs::caps_of). A broker asks whether
//! the process that sent it a message holds one right, and nothing else: a
//! launcher (SpawnWindow, Admin standing in, as `can_spawn_window`) or one
//! that may ask for installs (AppInstall). Never a mask, never another bit.
//! A process that started after the message was queued is not its sender:
//! a pid comes back only after the counter wraps, so a reused one is new.

use crate::capabilities::Capability;

pub(crate) const EPERM: i64 = -1;
pub(crate) const ESRCH: i64 = -3;
pub(crate) const EINVAL: i64 = -22;

/// The only rights a broker may ask about.
pub(crate) const ASKABLE: [u64; 2] = [Capability::SpawnWindow.bit(), Capability::AppInstall.bit()];

/// The process the pid names now: running, when it started (uptime ms) and its mask.
#[derive(Clone, Copy)]
pub(crate) struct Target {
    pub live: bool,
    pub started_ms: u64,
    pub caps: u64,
}

/// 1 or 0 for `bit` held by `target`, the sender of a message queued at
/// `sent_ms`; else EPERM to a non-broker, EINVAL for a bit not askable,
/// ESRCH for no live process, or one started after the message.
pub(crate) fn answer(broker: bool, bit: u64, target: Option<Target>, sent_ms: u64) -> i64 {
    if !broker {
        return EPERM;
    }
    if !ASKABLE.contains(&bit) {
        return EINVAL;
    }
    let Some(t) = target.filter(|t| t.live && t.started_ms <= sent_ms) else {
        return ESRCH;
    };
    let held =
        if bit == Capability::SpawnWindow.bit() { bit | Capability::Admin.bit() } else { bit };
    (t.caps & held != 0) as i64
}
