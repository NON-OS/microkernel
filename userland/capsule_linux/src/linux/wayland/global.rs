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

//! One global a client is told about (registry.rs), and finding it by name.

use super::object::Object;

pub struct Global {
    pub name: u32,
    pub interface: &'static [u8],
    pub version: u32,
    pub object: Object,
}

pub fn by_name(name: u32) -> Option<&'static Global> {
    super::registry::GLOBALS.iter().find(|g| g.name == name)
}
