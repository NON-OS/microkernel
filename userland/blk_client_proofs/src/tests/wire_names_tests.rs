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


//! Every name the client looks up for a disk on IDE or a SCSI adapter is the
//! handle a capsule registers, so a disk those drivers serve is offered to
//! install on. Only the first name of each driver is held by
//! scripts/check_blk_wire.py; this holds all of them.

use crate::driver::Driver;

const HANDLES: [&str; 6] = [
    include_str!("../../../capsule_driver_piix_ide/Capsule.mk"),
    include_str!("../../../capsule_driver_virtio_scsi/Capsule.mk"),
    include_str!("../../../capsule_driver_pvscsi/Capsule.mk"),
    include_str!("../../../capsule_driver_buslogic/Capsule.mk"),
    include_str!("../../../capsule_driver_mpt/Capsule.mk"),
    include_str!("../../../capsule_driver_storvsc/Capsule.mk"),
];

fn handle(mk: &str) -> &str {
    let line = mk.lines().find(|l| l.starts_with("CAPSULE_HANDLE")).expect("no handle");
    line.split(":=").nth(1).expect("no value").trim()
}

fn names(d: Driver) -> Vec<&'static str> {
    (0..d.instances()).map(|n| core::str::from_utf8(d.service(n).unwrap()).unwrap()).collect()
}

#[test]
fn each_ide_and_scsi_name_is_a_capsules_handle_and_each_handle_is_asked() {
    let asked: Vec<&str> = [Driver::Ide, Driver::Scsi].into_iter().flat_map(names).collect();
    let registered: Vec<&str> = HANDLES.iter().map(|mk| handle(mk)).collect();
    assert_eq!(asked, registered);
    assert_eq!(Driver::Scsi.service(Driver::Scsi.instances()), None, "no name past the last");
}

#[test]
fn ide_and_scsi_disks_speak_the_block_wire() {
    for d in [Driver::Ide, Driver::Scsi] {
        assert_eq!(d.magic(), Driver::Wire.magic(), "{d:?}");
        assert_eq!(d.controller(), None, "{d:?}: no controller counted missing");
    }
    assert_eq!((Driver::Ide.label(), Driver::Scsi.label()), (&b"IDE"[..], &b"SCSI"[..]));
}
