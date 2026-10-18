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

//! A write a unit refuses, a command the device never finishes, and the
//! wire status each failure becomes.

use crate::model::{Cfg, Lu, Rig};
use crate::ufs::disk::rw::{read, write};
use crate::ufs::error::{wire_status, UfsError};
use crate::ufs::facts::bounds::SCSI_MS;

#[test]
fn a_write_protected_unit_refuses_with_erofs() {
    let mut lu = Lu::new(0, 4096, 8);
    lu.write_protect = true;
    let rig = Rig::new(Cfg { lus: vec![lu], ..Cfg::default() });
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    let e = write(&rig.m, &ufs, &u, 0, 1, &[1; 4096]).unwrap_err();
    assert_eq!(e, UfsError::Sense { key: 0x7, asc: 0x27, ascq: 0 });
    assert_eq!(wire_status(e), -30);
}

#[test]
fn a_command_the_device_never_finishes_times_out_and_frees_the_slot() {
    let rig = Rig::new(Cfg::default());
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    rig.m.cfg.borrow_mut().scsi_hang = true;
    let start = rig.m.now.get();
    let mut b = vec![0u8; 4096];
    assert_eq!(read(&rig.m, &ufs, &u, 0, 1, &mut b), Err(UfsError::Timeout));
    assert!(rig.m.now.get() - start >= SCSI_MS);
    assert_eq!(rig.m.host.borrow().clears, vec![!1u32]);
    assert_eq!(wire_status(UfsError::Timeout), -110);
    assert_eq!(wire_status(UfsError::Sense { key: 0x5, asc: 0x21, ascq: 0 }), -22);
    assert_eq!(wire_status(UfsError::Ocs(7)), -5);
}
