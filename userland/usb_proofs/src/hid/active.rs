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

//! The keyboard layout the proofs type under: US, the layout the driver
//! starts on and keeps while the policy store is not up. The driver's own
//! `hid/active.rs` asks that store over IPC, which the host does not have.

use nonos_keymap::Layout;

pub fn current() -> Layout {
    Layout::from_index(0)
}

/// The driver cycles to the next layout and says so on the debug channel;
/// here the chord is consumed and the layout stays put.
pub fn cycle() -> Layout {
    current()
}
