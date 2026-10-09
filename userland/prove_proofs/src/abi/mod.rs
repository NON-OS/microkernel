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

//! libc's readers, unchanged, at `crate::abi`, where the capsule's own
//! `abi.rs` puts them.

#[path = "../../../libc/src/boot_slots/record.rs"]
pub mod boot_slots;
#[path = "../../../libc/src/enroll/frame.rs"]
pub mod enroll;

pub use boot_slots::{parse_boot_slots, BootSlot, PATH_LEN};
pub use enroll::split_public;
