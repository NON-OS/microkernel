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

//! The controller's I/O registers. A port that cannot be read answers all
//! ones, as a missing device does, so every wait on it ends at its bound.

pub trait Io {
    fn r16(&mut self, off: u16) -> u16;
    fn w16(&mut self, off: u16, v: u16);
    fn w32(&mut self, off: u16, v: u32);
}
