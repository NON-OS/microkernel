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

//! Directory contents: 32-byte slots, a short name each, with long-name
//! slots in front of one no 8.3 name spells.

mod encode;
mod long_name;
mod part;
mod short_name;
mod slot;

pub use encode::encode;
pub use short_name::NameError;

/// The slots a child named `name` takes in its directory: its short slot,
/// and the long ones in front when no 8.3 name spells it.
pub fn slots_for(name: &str) -> usize {
    match short_name::encode(name) {
        Ok(_) => 1,
        Err(_) => 1 + long_name::units(name).map_or(0, |u| long_name::slots(u.len())),
    }
}
