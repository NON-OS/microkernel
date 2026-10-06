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

//! What the desktop says, once per boot, about the store vfs loaded.
//!
//! vfs latches the first fault of the store's load as a code. 9 is also what
//! it latches when the store loaded with one damaged entry left out: the
//! store is there and works, so that is a warning saying what happened, not
//! "corrupted". Only a store that was read and could not be decoded (a short
//! reply, a bad length) is corrupted.
//!
//! The codes that mean no disk was read used to say nothing, as the normal
//! state of a live boot. But a stick carries its store: a stick boot whose
//! store did not load showed a desktop with no Linux programs, no Qwen and
//! no saved settings, and nothing said why. A disk that was there and did
//! not answer (2) or refused the read (7) is now said as a warning. No disk
//! at all (1), the ISO's normal state, is said as a notice of what this boot
//! lacks, which is true on either. Every line fits a toast whole
//! (TOAST_TEXT_MAX): the line for 9 was cut mid-sentence.

use super::NotifyLevel;

/// The line for store status `code`, if the desktop should say one.
pub fn store_word(code: u32) -> Option<(&'static [u8], NotifyLevel)> {
    match code {
        1 => Some((b"No NONOS disk found: no Linux programs this boot", NotifyLevel::Info)),
        2 => Some((b"Store disk did not answer; restart to try again", NotifyLevel::Warn)),
        7 => Some((b"The store's disk refused reads; restart to retry", NotifyLevel::Warn)),
        9 => Some((b"A damaged store entry left out; the rest loaded", NotifyLevel::Warn)),
        3 | 6 => Some((b"The capsule store could not be read", NotifyLevel::Error)),
        _ => None,
    }
}
