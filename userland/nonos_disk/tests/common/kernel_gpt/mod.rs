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

//! The kernel's GPT reader, compiled for the host from
//! `src/hardware/block_device/gpt/`, so the tests read what the installer
//! wrote with the code that reads it at boot. `read.rs`, the only part that
//! asks a driver, is left out.

#[path = "../../../../../src/hardware/block_device/gpt/array.rs"]
mod array;
#[path = "../../../../../src/hardware/block_device/gpt/boot.rs"]
mod boot;
#[path = "../../../../../src/hardware/block_device/gpt/bytes.rs"]
mod bytes;
#[path = "../../../../../src/hardware/block_device/gpt/choose.rs"]
pub mod choose;
#[path = "../../../../../src/hardware/block_device/gpt/crc32.rs"]
pub mod crc32;
#[path = "../../../../../src/hardware/block_device/gpt/entry.rs"]
mod entry;
#[path = "../../../../../src/hardware/block_device/gpt/fields.rs"]
mod fields;
#[path = "../../../../../src/hardware/block_device/gpt/fixed.rs"]
mod fixed;
#[path = "../../../../../src/hardware/block_device/gpt/header.rs"]
pub mod header;
#[path = "../../../../../src/hardware/block_device/gpt/layout.rs"]
pub mod layout;
#[path = "../../../../../src/hardware/block_device/gpt/mbr.rs"]
pub mod mbr;
#[path = "../../../../../src/hardware/block_device/gpt/places.rs"]
pub mod places;
#[path = "../../../../../src/hardware/block_device/gpt/table.rs"]
mod table;
#[path = "../../../../../src/hardware/block_device/gpt/which.rs"]
pub mod which;
#[path = "../../../../../src/hardware/block_device/gpt/window.rs"]
mod window;

pub(crate) use boot::boot_guid;
pub(crate) use choose::{choose, Held, Table};
pub(crate) use entry::{entries, Entry};
pub(crate) use fixed::FIXED;
pub(crate) use layout::{layout, Layout, Shown};
pub(crate) use places::{holds, places, Extent, Places};
pub(crate) use which::Which;
pub(crate) use window::{plan_at, store_at};
