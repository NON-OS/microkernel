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

//! NUMA facts. `MkAcpiProximity(segment, first << 8 | last, domain)`: the
//! ACPI capsule publishes a PCI root bridge's buses and its _PXM; only the
//! `acpi` service's owner may call it, and every record's `numa_node` is
//! filled again after. For anyone who may list devices: `MkNumaNodeOf(base,
//! len) -> domain` (ENOENT where SRAT names no memory, EXDEV across two
//! domains) and `MkNumaDistance(from, to) -> SLIT value` (ENOENT without a
//! SLIT or past its localities), and `MkDeviceNode(id) -> domain`, the
//! full domain the record's byte cannot name past 254 (ENOENT when
//! nothing names one). The allocator consumes none of this yet.

use crate::hardware::broker::acpi::numa_node::{node_of, BusRange, NodeError};
use crate::hardware::broker::acpi::numa_refresh::refresh;
use crate::hardware::broker::acpi::numa_store::{device_domain, distance, publish, regions};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{
    ERRNO_INVAL, ERRNO_NOENT, ERRNO_NOSPC, ERRNO_PERM, ERRNO_XDEV,
};

pub fn sys_acpi_proximity(segment: u64, buses: u64, domain: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    let (Ok(segment), Ok(buses), Ok(domain)) =
        (u16::try_from(segment), u16::try_from(buses), u32::try_from(domain))
    else {
        return ERRNO_INVAL;
    };
    let [last, first] = buses.to_le_bytes();
    if first > last {
        return ERRNO_INVAL;
    }
    if !publish(BusRange { segment, first, last, domain }) {
        return ERRNO_NOSPC;
    }
    refresh();
    0
}

pub fn sys_numa_node_of(base: u64, len: u64) -> i64 {
    match node_of(&regions(), base, len) {
        Ok(d) => i64::from(d),
        Err(NodeError::Outside) => ERRNO_NOENT,
        Err(NodeError::Spans) => ERRNO_XDEV,
    }
}

pub fn sys_numa_distance(from: u64, to: u64) -> i64 {
    let (Ok(from), Ok(to)) = (u32::try_from(from), u32::try_from(to)) else { return ERRNO_NOENT };
    distance(from, to).map_or(ERRNO_NOENT, i64::from)
}

pub fn sys_device_node(device_id: u64) -> i64 {
    device_domain(device_id).map_or(ERRNO_NOENT, i64::from)
}
