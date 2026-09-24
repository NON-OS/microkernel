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

use super::NotifyLevel;

pub const TOAST_TEXT_MAX: usize = 48;
pub const TOAST_LIFETIME_MS: i64 = 4000;

#[derive(Clone, Copy)]
pub struct Toast {
    pub text: [u8; TOAST_TEXT_MAX],
    pub len: usize,
    pub level: NotifyLevel,
    pub expires_at_ms: i64,
}

impl Toast {
    /// Text longer than the box holds is cut rather than refused.
    pub fn new(text: &[u8], level: NotifyLevel, now_ms: i64) -> Self {
        let len = text.len().min(TOAST_TEXT_MAX);
        let mut toast = Toast {
            text: [0; TOAST_TEXT_MAX],
            len,
            level,
            expires_at_ms: now_ms + TOAST_LIFETIME_MS,
        };
        toast.text[..len].copy_from_slice(&text[..len]);
        toast
    }
}
