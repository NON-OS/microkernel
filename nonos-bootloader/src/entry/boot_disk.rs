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

//! The store copy and the disk mirror, as the modules the kernel reads.
//!
//! The store module says, in its `reserved` field, whether the copy came
//! from the disk this loader was read from. A kernel that cannot drive that
//! disk then keeps no other: another NONOS disk, an installed one, would
//! give a stick's boot that disk's store, setup marker and data volume.

use nonos_boot::handoff::types::{
    Module, MODULE_KIND_DISK_MIRROR, MODULE_KIND_STORE, STORE_FROM_BOOT_DISK,
};
use nonos_boot::loader::file::{disk_mirror, store_copy};
use uefi::prelude::*;

pub(super) fn boot_disk(st: &SystemTable<Boot>) -> (Module, Module) {
    let bs = st.boot_services();
    let Some(((base, size), disk, own)) = store_copy(bs) else {
        return (Module::default(), Module::default());
    };
    let reserved = if own { STORE_FROM_BOOT_DISK } else { 0 };
    let store = Module { base, size, kind: MODULE_KIND_STORE, reserved };
    let mirror = match disk_mirror(bs, disk) {
        Some((base, size)) => Module { base, size, kind: MODULE_KIND_DISK_MIRROR, reserved: 0 },
        None => Module::default(),
    };
    (store, mirror)
}
