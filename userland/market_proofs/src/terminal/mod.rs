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

//! The Terminal's `market`: how it cuts a description to the screen.

#[path = "../../../capsule_terminal/src/command/builtin/market/wrap.rs"]
mod wrap;

pub fn wrap(text: &[u8], width: usize) -> alloc::vec::Vec<&[u8]> {
    wrap::wrap(text, width)
}
