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
//! One toast: what it says, how loud, and when it goes away.
//!
//! Every time here is on the uptime clock (`mk_uptime_ms`, read through
//! `server::toast_clock`), never the wall clock. The wall clock reads as an
//! error until the RTC has been read, and steps back when NTP corrects it (an
//! RTC kept in local time is hours ahead on many laptops): a toast stamped on
//! it stayed up for as long as the clock stood still, or for the length of
//! the step back.

use super::NotifyLevel;

pub const TOAST_TEXT_MAX: usize = 48;

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

#[derive(Clone, Copy)]
pub struct Toast {
    pub text: [u8; TOAST_TEXT_MAX],
    pub len: usize,
    pub level: NotifyLevel,
    /// When it was first shown. A repeat of it never moves this.
    pub shown_at_ms: i64,
    pub expires_at_ms: i64,
}

impl Toast {
    /// Text longer than the box holds is cut rather than refused.
    pub fn new(text: &[u8], level: NotifyLevel, now: UptimeMs, lifetime_ms: i64) -> Self {
        let len = text.len().min(TOAST_TEXT_MAX);
        let mut toast = Toast {
            text: [0; TOAST_TEXT_MAX],
            len,
            level,
            shown_at_ms: now.0,
            expires_at_ms: now.0.saturating_add(lifetime_ms),
        };
        toast.text[..len].copy_from_slice(&text[..len]);
        toast
    }

    /// Whether this says `text` at `level`, cut as a toast cuts it.
    pub fn says(&self, text: &[u8], level: NotifyLevel) -> bool {
        self.level == level && self.text[..self.len] == text[..text.len().min(TOAST_TEXT_MAX)]
    }

    /// Whether its time is up at `now`. A reading behind the time it was
    /// shown (a clock that went back) ends it too, rather than holding it
    /// until the clock catches up.
    pub fn over(&self, now: UptimeMs) -> bool {
        now.0 >= self.expires_at_ms || now.0 < self.shown_at_ms
    }
}
