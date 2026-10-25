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

//! The installer's client for driver.ufs0: the block wire it speaks, held to
//! the capsule's own Capsule.mk and host table, and the bus it names.

use super::pci::{classify, Controller};
use super::table::{Driver, ALL};
use super::ufs_hosts::HOSTS;
use super::wire_identity_parse::parse_wire_identity;
use nonos_blk_wire::{encode_identify, MEDIUM_UFS};

#[test]
fn the_installer_speaks_driver_ufs0s_wire() {
    let mk = include_str!("../../../capsule_driver_ufs/Capsule.mk");
    let handle = mk.lines().find_map(|l| l.strip_prefix("CAPSULE_HANDLE")).expect("handle");
    let handle = handle.trim().trim_start_matches(":=").trim();
    assert_eq!(Driver::Ufs.service(0), Some(handle.as_bytes()));
    assert_eq!((Driver::Ufs.service(1), Driver::Ufs.instances()), (None, 1));
    // One wire: the capsule serves the frames driver.sdhci0 does.
    assert_eq!(Driver::Ufs.magic(), Driver::Wire.magic());
    let (u, w) = (Driver::Ufs.ops(), Driver::Wire.ops());
    assert_eq!([u.capacity, u.read, u.write, u.flush], [w.capacity, w.read, w.write, w.flush]);
}

#[test]
fn every_host_the_capsule_binds_is_a_ufs_controller_and_only_its_own() {
    for &(vendor, device) in HOSTS {
        let c = classify(vendor, device, 0x01, 0x09, 0x00);
        assert_eq!(c, Some(Controller::Ufs), "{vendor:04x}:{device:04x}");
    }
    for d in ALL {
        assert_eq!(d.serves(Controller::Ufs), d == Driver::Ufs, "{d:?}");
    }
}

#[test]
fn a_ufs_part_is_named_and_is_no_emmc() {
    let body = encode_identify(b"QEMU    QEMU HARDDISK", b"0123456789", MEDIUM_UFS).unwrap();
    let id = parse_wire_identity(&body).expect("taken");
    assert_eq!((id.model_str(), id.serial_str()), ("QEMU    QEMU HARDDISK", "0123456789"));
    assert!(!id.emmc, "a UFS part said eMMC");
}

#[test]
fn each_driver_names_its_own_bus() {
    let named: Vec<&[u8]> = ALL.iter().map(|d| d.label()).collect();
    assert_eq!(named, [&b"NVMe"[..], b"SATA", b"eMMC", b"UFS", b"IDE", b"SCSI", b"virtio", b"SD"]);
}

#[test]
fn the_capacity_reply_buffer_holds_the_block_wires_geometry() {
    // A GEOMETRY reply is 44 bytes; a receive sized for an 8-byte count
    // refuses it, and the installer opens no eMMC or UFS disk (R7).
    use nonos_blk_wire::{GEOMETRY_LEN, HDR_LEN, STATUS_LEN};
    let room = std::hint::black_box(HDR_LEN + STATUS_LEN + super::dialect::CAPACITY_BODY_MAX);
    // driver.sdhci0 and driver.ufs0 both answer nonos_blk_wire's GEOMETRY;
    // the other dialects answer an 8-byte count.
    for body in [GEOMETRY_LEN, 8] {
        assert!(HDR_LEN + STATUS_LEN + body <= room, "{body}-byte body");
    }
}
