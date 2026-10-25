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

//! The sink's bus state between the driver's register accesses.

#[derive(Clone, Copy)]
pub struct Cmd {
    pub addr: u32,
    pub read: bool,
    pub len: usize,
}

#[derive(Default)]
pub struct Bus {
    pub sel: u32,
    pub data: u32,
    pub segment: u8,
    pub pointed: bool,
    pub cmd: Option<Cmd>,
    pub stream: Vec<u8>,
    pub words: usize,
    pub slowed: bool,
    pub ci_last: Vec<u8>,
}
