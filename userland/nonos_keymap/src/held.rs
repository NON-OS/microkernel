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

/// Keys held down, each with the code its press was posted with.
///
/// A key's code is its character under the modifiers and layout of the
/// moment, so resolved again at the release it can differ from its press:
/// Shift+A let go after Shift comes up is 'a'. The input router pairs a
/// release with its press by code, and a client that tracks held keys (a
/// Linux app under the Wayland bridge, which repeats them itself) only lets
/// go of the code it was pressed with. So a release is posted with the
/// code its press went down with, and a repeat that now resolves to another
/// code first releases the old one.
///
/// `key` names the physical key (the driver's own number for it, never 0).
/// A code of 0 marks a press the driver consumed (the layout chord): its
/// repeats and its release post nothing.
#[derive(Clone, Copy)]
pub struct HeldKeys {
    keys: [(u32, u32); MAX_HELD],
}

/// More than a keyboard reports at once (six keys and the modifiers).
const MAX_HELD: usize = 16;

impl HeldKeys {
    pub const fn new() -> Self {
        Self { keys: [(0, 0); MAX_HELD] }
    }

    /// The code `key` is held with, if it is held.
    pub fn code(&self, key: u32) -> Option<u32> {
        self.keys.iter().find(|&&(k, _)| k == key && key != 0).map(|&(_, c)| c)
    }

    /// `key` went down (or repeated) resolving to `code`. Returns the code it
    /// was held with when that is another one, for the caller to release
    /// first. A full table keeps nothing; that key's release then resolves on
    /// its own, as before.
    pub fn press(&mut self, key: u32, code: u32) -> Option<u32> {
        if key == 0 {
            return None;
        }
        if let Some(slot) = self.keys.iter_mut().find(|(k, _)| *k == key) {
            let before = slot.1;
            slot.1 = code;
            return (before != code && before != 0).then_some(before);
        }
        if let Some(slot) = self.keys.iter_mut().find(|(k, _)| *k == 0) {
            *slot = (key, code);
        }
        None
    }

    /// `key` came up: the code it was pressed with, if it was held.
    pub fn release(&mut self, key: u32) -> Option<u32> {
        let slot = self.keys.iter_mut().find(|(k, _)| *k == key && key != 0)?;
        let code = slot.1;
        *slot = (0, 0);
        Some(code)
    }

    /// Whether a key other than `key` is held that `same` picks out: the
    /// other Shift, Ctrl or Meta still down when one of a pair is let go.
    pub fn other_held(&self, key: u32, same: impl Fn(u32) -> bool) -> bool {
        self.keys.iter().any(|&(k, _)| k != 0 && k != key && same(k))
    }

    /// What one key event posts, given the code it resolves to now (0 for a
    /// key the layout leaves empty): first a release for the code the key
    /// was held with, when a repeat now resolves to another, then the event
    /// itself with the code it pairs with.
    pub fn event(&mut self, key: u32, is_release: bool, resolved: u32) -> KeyPosts {
        if is_release {
            let code = match self.release(key) {
                Some(code) => code,
                None => resolved,
            };
            return KeyPosts { release_first: None, code: nonzero(code) };
        }
        if resolved == 0 {
            // Held as something the layout no longer gives: let that go.
            let before = self.release(key).and_then(nonzero);
            return KeyPosts { release_first: before, code: None };
        }
        KeyPosts { release_first: self.press(key, resolved), code: Some(resolved) }
    }
}

/// The codes one key event posts, in order: a release of `release_first`,
/// then the event itself with `code`. None posts nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyPosts {
    pub release_first: Option<u32>,
    pub code: Option<u32>,
}

fn nonzero(code: u32) -> Option<u32> {
    (code != 0).then_some(code)
}

impl Default for HeldKeys {
    fn default() -> Self {
        Self::new()
    }
}
