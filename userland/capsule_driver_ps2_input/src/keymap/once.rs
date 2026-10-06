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

//! The keys a hold must not repeat.
//!
//! The keyboard repeats a held key as more make codes. Mute held would flip
//! the sound on and off at the repeat rate, and the power key held would ask
//! for a shutdown thirty times a second, so their repeats are dropped here and
//! each press acts once, as the USB driver's repeat leaves them out too.

use super::set1::{KEYCODE_MUTE, KEYCODE_POWER};

/// Whether `keycode` acts once per press, its repeats posting nothing.
pub fn acts_once(keycode: u32) -> bool {
    matches!(keycode, KEYCODE_MUTE | KEYCODE_POWER)
}
