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

//! Operation regions, field units and buffer fields at run time.

mod access;
mod bridge;
mod buffer_field;
mod buffer_field_io;
mod buffer_field_make;
mod connection;
mod data_region;
mod global_lock;
mod gpio;
mod index_bank;
mod lock_word;
mod pci_path;
mod read;
mod reg;
mod region;
mod region_io;
mod rules;
mod target;
mod transaction;
mod value;
mod write;

pub(crate) use region_io::Io;
