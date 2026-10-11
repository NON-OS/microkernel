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

//! A request the host never finishes, and a clear it never takes (the
//! the maintainers' review): the slot is the host's for good, so no later
//! request is issued into it and no write fills the data region an old
//! READ may still complete into.

use crate::model::{Cfg, Rig};
use crate::ufs::disk::rw::{read, write};
use crate::ufs::error::UfsError;

#[test]
fn a_slot_the_host_kept_is_never_used_again() {
    let rig = Rig::new(Cfg::default());
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    {
        let mut c = rig.m.cfg.borrow_mut();
        c.scsi_hang = true;
        c.clear_never = true;
    }
    let mut b = vec![0u8; 4096];
    assert_eq!(read(&rig.m, &ufs, &u, 0, 1, &mut b), Err(UfsError::Timeout));
    let utrds = rig.m.host.borrow().utrds.len();
    // The device would answer now; the slot is still not the driver's.
    rig.m.cfg.borrow_mut().scsi_hang = false;
    assert_eq!(read(&rig.m, &ufs, &u, 0, 1, &mut b), Err(UfsError::Host));
    let marker = [0xA5u8; 4096];
    assert_eq!(write(&rig.m, &ufs, &u, 0, 1, &marker), Err(UfsError::Host));
    assert_eq!(rig.m.host.borrow().utrds.len(), utrds, "nothing issued after");
    let mut data = vec![0u8; 16];
    ufs.ctx.lists.data.get(0, &mut data);
    assert!(data.iter().all(|b| *b != 0xA5), "the bounce buffer was filled");
}

#[test]
fn a_clear_that_takes_frees_the_slot() {
    let rig = Rig::new(Cfg::default());
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    rig.m.cfg.borrow_mut().scsi_hang = true;
    let mut b = vec![0u8; 4096];
    assert_eq!(read(&rig.m, &ufs, &u, 0, 1, &mut b), Err(UfsError::Timeout));
    rig.m.cfg.borrow_mut().scsi_hang = false;
    assert_eq!(read(&rig.m, &ufs, &u, 0, 1, &mut b), Ok(()));
}
