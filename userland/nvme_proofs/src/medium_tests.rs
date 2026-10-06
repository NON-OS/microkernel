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

//! Who the driver serves: the medium, and everything that names the disk,
//! answers only the kernel's client (pid 0) or a sender the kernel says holds
//! StoreWrite. Any other capsule could otherwise read or overwrite every
//! sector, another system's partitions among them.

use crate::medium_rule::allows;
use crate::protocol::{
    OP_CAPACITY, OP_FLUSH, OP_HEALTHCHECK, OP_READ_BLOCKS, OP_WRITE_BLOCKS, OP_IDENTIFY_CONTROLLER, OP_IDENTIFY_NAMESPACE, OP_SMART_HEALTH, OP_CONTROLLER_INFO,
};

const STRANGER: u32 = 7;

#[test]
fn a_capsule_without_store_write_reaches_nothing_on_the_disk() {
    for op in [OP_READ_BLOCKS, OP_WRITE_BLOCKS, OP_FLUSH, OP_CAPACITY, OP_IDENTIFY_CONTROLLER, OP_IDENTIFY_NAMESPACE, OP_SMART_HEALTH, OP_CONTROLLER_INFO, 0xffff] {
        assert!(!allows(op, STRANGER, false), "op {op:#x} answered a stranger");
    }
}

#[test]
fn the_kernel_client_is_never_refused() {
    for op in [OP_READ_BLOCKS, OP_WRITE_BLOCKS, OP_FLUSH, OP_CAPACITY, OP_IDENTIFY_CONTROLLER, OP_IDENTIFY_NAMESPACE, OP_SMART_HEALTH, OP_CONTROLLER_INFO] {
        assert!(allows(op, 0, false), "op {op:#x} refused the kernel");
    }
}

#[test]
fn a_store_write_holder_reaches_the_medium() {
    for op in [OP_READ_BLOCKS, OP_WRITE_BLOCKS, OP_FLUSH, OP_CAPACITY] {
        assert!(allows(op, STRANGER, true), "op {op:#x} refused the installer");
    }
}

#[test]
fn a_health_check_stays_open() {
    assert!(allows(OP_HEALTHCHECK, STRANGER, false));
}
