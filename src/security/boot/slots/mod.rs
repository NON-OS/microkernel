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

//! The two boot slots of the device proof's witness, as this boot has them:
//! the bootloader's and the kernel's context digest, path and root. Read by
//! `MkBootSlots`, so nonos.prove never parses an image.

mod footer;
mod read;
pub mod record;
pub mod slot;

pub use read::boot_slots_record;
pub use record::RECORD_LEN;
