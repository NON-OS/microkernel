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

//! Every disk behind an IDE controller or a SCSI host is offered, not only
//! its first: each request names its unit, as many units are asked as the
//! driver may hold, and a unit with no disk is no fault. units_walk_tests
//! holds the walk over the units and the words that tell two disks apart.

use crate::driver::Driver;
use crate::error::BlkError;
use crate::medium::{held, E_NOMEDIUM, E_NXIO};
use crate::wire::encode_request;

const IDE_DISKS: &str = include_str!("../../../capsule_driver_piix_ide/src/disk/mod.rs");
const SCSI_UNITS: &str = include_str!("../../../nonos_scsi_serve/src/units.rs");
const ERRNOS: [&str; 2] = [
    include_str!("../../../capsule_driver_piix_ide/src/wire/errno.rs"),
    include_str!("../../../nonos_scsi_serve/src/wire/errno.rs"),
];

#[test]
fn the_unit_rides_in_the_header() {
    let tx = encode_request(0x4E42_5731, 2, 3, 9, &[]);
    assert_eq!(&tx[10..12], &3u16.to_le_bytes(), "the unit, where the drivers read it");
    assert_eq!(&tx[8..10], &[0, 0], "the flags stay clear");
    assert_eq!(&tx[12..16], &9u32.to_le_bytes(), "the request id after it");
}

#[test]
fn as_many_units_as_each_driver_holds() {
    assert!(IDE_DISKS.contains(&format!("pub const UNITS: usize = {};", Driver::Ide.units())));
    let scsi = format!("pub const MAX_UNITS: usize = {};", Driver::Scsi.units());
    assert!(SCSI_UNITS.contains(&scsi));
    for d in [Driver::Nvme, Driver::Ahci, Driver::VirtioBlk, Driver::Wire, Driver::Ufs] {
        assert_eq!(d.units(), 1, "{d:?} serves one disk per service");
    }
}

#[test]
fn a_unit_with_no_disk_is_not_a_fault_where_there_are_several() {
    for errno in ERRNOS {
        assert!(
            errno.contains(&format!("pub const E_NXIO: i32 = {E_NXIO};")),
            "the drivers' ENXIO"
        );
    }
    let nxio: Result<u8, BlkError> = Err(BlkError::Status(E_NXIO));
    assert_eq!(held(Driver::Scsi, nxio), Ok(None));
    assert_eq!(held(Driver::Ide, nxio), Ok(None));
    assert_eq!(held(Driver::Wire, nxio), nxio.map(Some), "eMMC's only unit missing");
    assert_eq!(held(Driver::Rtsx, Err::<u8, _>(BlkError::Status(E_NOMEDIUM))), Ok(None));
    assert_eq!(held(Driver::Scsi, Ok(7u8)), Ok(Some(7)));
}
