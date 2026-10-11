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

//! A request the host ends with a failing Overall Command Status is that
//! failure, whatever the response area holds.

use crate::model::{Cfg, Rig};
use crate::ufs::disk::rw::read;
use crate::ufs::error::{wire_status, UfsError};

#[test]
fn a_failing_ocs_is_the_requests_error() {
    let rig = Rig::new(Cfg::default());
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    // OCS_PEER_COMM_FAILURE.
    rig.m.cfg.borrow_mut().scsi_ocs = 0x05;
    let mut b = vec![0u8; 4096];
    assert_eq!(read(&rig.m, &ufs, &u, 0, 1, &mut b), Err(UfsError::Ocs(5)));
    assert_eq!(wire_status(UfsError::Ocs(5)), -5);
}
