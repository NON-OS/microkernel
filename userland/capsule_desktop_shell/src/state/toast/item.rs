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

//! A toast's text, level and times.

use super::super::NotifyLevel;
use super::times::UptimeMs;

pub const TOAST_TEXT_MAX: usize = 48;

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
