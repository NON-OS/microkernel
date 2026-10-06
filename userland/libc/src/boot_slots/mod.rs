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

//! This boot's two slots of the device proof's witness, the bootloader's and
//! the kernel's, as `MkBootSlots` reports them. For the capsule that proves.

mod call;
mod record;

pub use call::{boot_slots, mk_boot_slots};
pub use record::{parse_boot_slots, BootSlot, BootSlots, BOOT_SLOTS_LEN, PATH_LEN};
