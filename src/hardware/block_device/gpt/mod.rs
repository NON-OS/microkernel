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

//! The GPT a disk carries: read, checked, and searched for NONOS's own
//! partitions and the one the machine booted from. Pure but for `read`.

mod array;
mod boot;
mod bytes;
mod choose;
mod crc32;
mod entry;
mod fields;
mod fixed;
mod header;
mod layout;
mod mbr;
mod places;
mod read;
mod table;
mod which;
mod window;

pub(crate) use boot::boot_guid;
pub(crate) use choose::Held;
pub(crate) use fixed::FIXED;
pub(crate) use layout::{layout, Layout, Shown};
pub(crate) use places::holds;
pub use places::{Extent, Places};
pub(crate) use read::table;
pub(crate) use window::{plan_at, store_at};
