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

//! How long toasts stay up, on the uptime clock.

/// How long a notice ("Terminal opened", "Files closed", "opening a new
/// window", an app's own notify, a refusal) stays up, from the moment it was
/// first shown. The one lifetime every transient notice has.
pub const TOAST_LIFETIME_MS: i64 = 2500;

/// How long a notice said once per boot about the system itself (the
/// capsule store damaged or unreadable) stays up. Longer by design so it is
/// not missed while the desktop comes up, still bounded, and a press on the
/// panel dismisses it like any other.
pub const TOAST_HELD_MS: i64 = 10_000;

/// Milliseconds on the uptime clock. The queue takes no other time, so a
/// wall clock reading is not handed to it by mistake.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct UptimeMs(pub i64);
