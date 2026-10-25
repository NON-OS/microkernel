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

//! The recovery words as text, zeroed when dropped.

use alloc::string::String;

use zeroize::Zeroize;

/* The phrase, space separated. Zeroed when dropped. */
pub struct Words(pub(super) String);

impl Words {
    pub fn text(&self) -> &str {
        &self.0
    }
}

impl Drop for Words {
    fn drop(&mut self) {
        // Every byte, the spare capacity too, through zeroize's volatile writes.
        self.0.zeroize();
    }
}
