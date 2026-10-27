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

//! A transfer the data region cannot hold fails as Bounce. A write is never
//! sent, since the WRITE would carry whatever the region last held; a read
//! never reports bytes it could not copy out.

use crate::model::{Cfg, Rig};
use crate::ufs::disk::rw::{read, write};
use crate::ufs::error::{wire_status, UfsError};

#[test]
fn a_write_past_the_data_region_is_never_sent() {
    let rig = Rig::new(Cfg::default());
    let mut ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    write(&rig.m, &ufs, &u, 3, 1, &[0x11; 4096]).expect("one block fits");
    ufs.ctx.lists.data.len = 2048;
    let sent = rig.m.host.borrow().utrds.len();
    assert_eq!(write(&rig.m, &ufs, &u, 3, 1, &[0x22; 4096]), Err(UfsError::Bounce));
    assert_eq!(rig.m.host.borrow().utrds.len(), sent, "a WRITE was sent");
    assert_eq!(rig.m.cfg.borrow().lus[0].block(3), vec![0x11; 4096]);
    assert_eq!(wire_status(UfsError::Bounce), -5);
}

#[test]
fn a_read_past_the_data_region_reports_no_bytes() {
    let rig = Rig::new(Cfg::default());
    let mut ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    ufs.ctx.lists.data.len = 2048;
    let mut out = vec![0xAAu8; 4096];
    assert_eq!(read(&rig.m, &ufs, &u, 3, 1, &mut out), Err(UfsError::Bounce));
}
