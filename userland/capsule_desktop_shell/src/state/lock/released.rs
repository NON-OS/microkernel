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

//! Input the router sent while locked, or while the passphrase prompt was
//! up, and the shell reads after the unlock or the prompt's close. The shell
//! waits on the keyring while it checks or sets the passphrase, and what the
//! person typed or clicked meanwhile waits in its inbox; read once the lock
//! or the prompt is let go, a press would act on the desk or the dock under
//! it and a chord would run. A frame posted at or before the release is the
//! lock's and is dropped. The router keeps the same rule for what is still
//! in the kernel's ring after an unlock (route/session_lock.rs), so none of
//! it reaches a window.

const NS_PER_MS: u64 = 1_000_000;
/// Where a router frame carries its event's post time, kernel-written.
const POSTED_AT: usize = 32;

/// The release at uptime `now_ms`, as the nanoseconds put on an event.
pub fn released_at(now_ms: i64) -> u64 {
    u64::try_from(now_ms).unwrap_or(0).saturating_mul(NS_PER_MS)
}

/// The post time of a router frame; 0, no time, when it carries none.
pub fn posted_ns(frame: &[u8]) -> u64 {
    frame
        .get(POSTED_AT..POSTED_AT + 8)
        .and_then(|b| b.try_into().ok())
        .map(u64::from_le_bytes)
        .unwrap_or(0)
}

/// Whether a frame posted at `posted_ns` is the lock's after a release at
/// `released_ns`: posted in or before the release's millisecond. Nothing is
/// before the first release. A frame with no time is taken as posted now,
/// as the router takes it (route/session_lock.rs posted_locked), so the
/// desktop never stops answering for want of a clock.
pub fn is_locks(released_ns: Option<u64>, posted_ns: u64) -> bool {
    released_ns.is_some_and(|at| posted_ns != 0 && posted_ns / NS_PER_MS <= at / NS_PER_MS)
}
