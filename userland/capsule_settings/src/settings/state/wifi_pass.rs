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

//! The passphrase editor for a secured network: open or not, shown as typed
//! or as stars, and what is typed.

use super::edit_buffer::EditBuffer;

pub struct WifiPass {
    /// Whether the passphrase editor is open for the selected secured network.
    pub active: bool,
    /// The passphrase is drawn as typed rather than as stars (Tab).
    pub shown: bool,
    /// The passphrase being typed for a secured network.
    pub text: EditBuffer,
}

impl WifiPass {
    pub fn new() -> Self {
        WifiPass { active: false, shown: false, text: EditBuffer::empty() }
    }
}
