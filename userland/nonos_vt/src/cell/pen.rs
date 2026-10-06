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

//! The pen: what a program last set, which every cell it writes takes.

use crate::color::Color;

/// What newly written and erased cells take from the program.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Pen {
    pub fg: Color,
    pub bg: Color,
    pub attr: u16,
    pub link: u16,
}
