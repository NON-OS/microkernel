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

//! A channel's record names its relid in `irq_source`, the field a driver
//! reads to learn which channel it holds and which event flag is its own.
//! Read as text: the broker's registration and the call that makes it.

/// The source without whitespace, so formatting never breaks a match.
fn code(rel: &str) -> String {
    let path = format!("{}/../../src/{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

#[test]
fn a_channel_record_carries_its_relid_as_its_irq_source() {
    let broker = code("hardware/broker/vmbus.rs");
    let head = "pubfnregister_channel(device:u16,relid:u32)->Option<u64>{";
    let at = broker.find(head).expect("register_channel takes the relid");
    let body = &broker[at..(at + 400).min(broker.len())];
    assert!(body.contains("irq_source:relid,"), "the relid in irq_source");
    let call = code("syscall/microkernel/hv_vmbus.rs");
    assert!(
        call.contains("register_vmbus_channel(device,relid)"),
        "MkHvChannel registers the record with the relid it binds"
    );
}

/// The source of a userland file without whitespace.
fn user_code(rel: &str) -> String {
    let path = format!("{}/../{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// A class's page ceiling, read from dma/limits.rs.
fn ceiling(limits: &str, class: &str) -> u64 {
    let at = limits.find(&format!("ids::{class}=>")).unwrap_or_else(|| panic!("no {class} row"));
    let rest = &limits[at + class.len() + 7..];
    rest[..rest.find(',').unwrap()].parse().unwrap()
}

#[test]
fn a_channel_record_is_classed_by_its_interface_so_its_driver_can_map_its_buffers() {
    let broker = code("hardware/broker/vmbus.rs");
    assert!(broker.contains("CHANNEL_SCSI|CHANNEL_IDE=>ids::BLOCK,"));
    assert!(broker.contains("CHANNEL_NETWORK=>ids::NETWORK,"));
    assert!(broker.contains("_=>ids::OTHER,"));
    assert!(broker.contains("class:channel_class(device),"), "the record takes its interface's class");
    let limits = code("hardware/broker/dma/limits.rs");
    // storvsc maps one grant: two 32-page rings and a 64 KiB buffer.
    let layout = user_code("capsule_driver_storvsc/src/constants/layout.rs");
    assert!(layout.contains("pubconstRING_BYTES:usize=64*PAGE;"));
    assert!(layout.contains("pubconstDATA_BYTES:usize=0x1_0000;"));
    let storvsc_pages = 64 + 0x1_0000 / 4096;
    assert!(storvsc_pages <= ceiling(&limits, "BLOCK"), "storvsc's grant within a block device's");
    assert!(limits.contains("FALLBACK_PAGES:u64=16;"));
    assert!(storvsc_pages > 16, "the fallback ceiling, OTHER's, would refuse it");
}
