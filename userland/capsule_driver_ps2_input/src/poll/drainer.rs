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

use super::prefix::Prefix;
use crate::keymap::{LED_CAPS, LED_NUM};

pub struct Drainer {
    pub(super) prefix: Prefix,
    pub(super) mods: u16,
    /// The lock keys as the keyboard's LED byte (keymap/locks.rs). Caps Lock
    /// is a toggle, not a held modifier, so it lives here and is folded into
    /// the published flags per event. Num Lock starts on: the keypad types
    /// its digits (keymap/keypad.rs), and the first LED sync shows that.
    pub locks: u8,
    // The code each held key went down with, so its release carries it.
    pub(super) held: nonos_keymap::HeldKeys,
    /// The keyboard's last ACK or RESEND, for the command that waits on it.
    pub reply: Option<u8>,
}

impl Drainer {
    pub const fn new() -> Self {
        Self {
            prefix: Prefix::IDLE,
            mods: 0,
            locks: LED_NUM,
            held: nonos_keymap::HeldKeys::new(),
            reply: None,
        }
    }

    pub(super) fn caps(&self) -> bool {
        self.locks & LED_CAPS != 0
    }
}
