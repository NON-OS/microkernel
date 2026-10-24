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

//! The queue and what it is read for: the live toasts, the next expiry and
//! the generation.

use super::super::toast::{Toast, UptimeMs};

pub const MAX_TOASTS: usize = 3;
pub struct ToastQueue {
    pub(super) entries: [Option<Toast>; MAX_TOASTS],
    /// Bumped on every change to what the panel shows, so the chrome is
    /// repainted when, and only when, its toasts are out of date.
    pub(super) generation: u32,
}

impl ToastQueue {
    pub const fn new() -> Self {
        Self { entries: [None; MAX_TOASTS], generation: 0 }
    }

    /// Changes with every push, expiry and dismissal, never otherwise.
    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|e| e.is_none())
    }

    pub fn iter_live(&self) -> impl Iterator<Item = &Toast> {
        self.entries.iter().filter_map(|e| e.as_ref())
    }

    /// When the next toast's time is up, so the serve loop wakes for it
    /// whether or not anything else happens. None with nothing up.
    pub fn next_expiry(&self) -> Option<UptimeMs> {
        self.entries.iter().flatten().map(|t| UptimeMs(t.expires_at_ms)).min()
    }
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self::new()
    }
}
