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

//! The one word the window may pass the core's first start: the person's
//! display name, as the kernel admits it (app_launch_rule.rs): the prefix,
//! then the name, at most WORD_MAX bytes, no NUL and no control byte.

use alloc::string::String;

use super::start::{Phase, Start};

pub const PREFIX: &str = "--user-display-name=";
pub const WORD_MAX: usize = 96;
/// The longest name, in bytes, the word has room for.
pub const NAME_MAX: usize = WORD_MAX - PREFIX.len();

/// The word for `name`, its ends trimmed; None when that is empty, too long
/// or holds a control character.
pub fn word(name: &str) -> Option<String> {
    let name = name.trim();
    let fits = !name.is_empty() && name.len() <= NAME_MAX;
    (fits && !name.chars().any(char::is_control)).then(|| [PREFIX, name].concat())
}

impl Start {
    /// The person gave `name`; false, and nothing done, when it cannot be one.
    pub fn named(&mut self, name: &str) -> bool {
        let Some(w) = word(name).filter(|_| self.phase == Phase::Ask) else { return false };
        self.phase = Phase::Launch(Some(w));
        true
    }
}
