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

//! What the pointer is in the middle of.

use nonos_vt::input::Button;
use nonos_vt::Pos;

#[derive(Default)]
pub struct Pointer {
    /// A left-button drag is choosing text: from where, and as a block.
    pub selecting: bool,
    pub start: Pos,
    pub block: bool,
    /// Counting presses in one place for double and triple clicks.
    pub clicks: u8,
    pub last_ns: u64,
    pub last_pos: Pos,
    /// The button held, for motion reports to a program.
    pub held: Option<Button>,
}
