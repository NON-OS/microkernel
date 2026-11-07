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

//! The windows the fixtures are valid in, opened from the clock they are
//! made at. Each opens an hour early, so a guest whose clock is a little
//! behind the machine that made them still falls inside.

use crate::kit::roles_build::HOUR;
use nsh_roles::MAX_REVOCATION_AGE_MS;

/// How early every window opens.
pub const EARLY_MS: u64 = HOUR;
/// How long the user's base grant runs.
pub const GRANT_MS: u64 = 7 * 24 * HOUR;
/// How long the host's enrolment runs.
pub const ENROL_MS: u64 = 30 * 24 * HOUR;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    /// When the role table and revocation list were issued and the grant
    /// opens.
    pub opens_ms: u64,
    pub grant_ends_ms: u64,
    pub enrol_ends_ms: u64,
}

impl Window {
    /// The windows for fixtures made at `now_ms`, or `None` for a clock
    /// too early or too late to open them at.
    pub fn at(now_ms: u64) -> Option<Window> {
        Some(Window {
            opens_ms: now_ms.checked_sub(EARLY_MS)?,
            grant_ends_ms: now_ms.checked_add(GRANT_MS)?,
            enrol_ends_ms: now_ms.checked_add(ENROL_MS)?,
        })
    }

    /// The last moment the revocation list is fresh. The table never trusts
    /// a list older than `MAX_REVOCATION_AGE_MS`, so a host serving these
    /// fixtures past it admits no one until they are made again.
    pub fn list_stale_ms(&self) -> u64 {
        self.opens_ms.saturating_add(MAX_REVOCATION_AGE_MS)
    }
}
