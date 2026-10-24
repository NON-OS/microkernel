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

//! The device: NOP OUT answered, retried or timed out, and
//! fDeviceInit set and seen cleared within its bound.

use crate::model::{Cfg, Rig};
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::{DEVICE_INIT_MS, NOP_MS};
use crate::ufs::facts::upiu::RESPONSE;

/// A NOP that times out is not retried, and its slot is cleared through
/// UTRLCLR (a 0 for slot 0), so the doorbell is free again.
#[test]
fn a_silent_device_ends_at_the_first_nop_with_its_slot_cleared() {
    let rig = Rig::new(Cfg { nop_hang: true, ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::Timeout));
    let h = rig.m.host.borrow();
    assert_eq!(h.utrds.len(), 1);
    assert_eq!(h.clears, vec![!1u32]);
    assert_eq!(h.regs[(crate::ufs::facts::hci::UTRLDBR / 4) as usize], 0);
    assert!(rig.m.now.get() >= NOP_MS);
    assert!(rig.m.said("NOP OUT failed: timed out"));
}

#[test]
fn a_wrong_answer_is_retried_up_to_five_nops() {
    let rig = Rig::new(Cfg { nop_wrong: 2, ..Cfg::default() });
    assert!(rig.up(false).is_ok());
    assert_eq!(rig.m.dev.borrow().nops, 3);
    let rig = Rig::new(Cfg { nop_wrong: 5, ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::Upiu(RESPONSE)));
    assert_eq!(rig.m.dev.borrow().nops, 5);
}

#[test]
fn device_init_is_waited_out_and_bounded() {
    let rig = Rig::new(Cfg { init_reads: 200, ..Cfg::default() });
    assert!(rig.up(false).is_ok());
    assert!(!rig.m.dev.borrow().init_flag);
    let rig = Rig::new(Cfg { init_reads: u32::MAX, ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::Timeout));
    assert!(rig.m.now.get() >= DEVICE_INIT_MS);
    assert!(rig.m.said("fDeviceInit failed: timed out"));
}

/// UFSHCI 1.1 numbers itself 0x0001_0100 and takes command type 2 for
/// device management and 0 for SCSI (facts::utp, facts::hci::VER_1X).
#[test]
fn a_ufshci_1_1_host_gets_the_older_command_types() {
    let rig = Rig::new(Cfg { ver: 0x0001_0100, ..Cfg::default() });
    assert!(rig.up(false).is_ok());
    let h = rig.m.host.borrow();
    let types: Vec<u32> = h.utrds.iter().map(|(t, _)| *t).collect();
    assert_eq!(&types[..2], &[2, 2], "NOP OUT, then the fDeviceInit set");
    assert!(types[2..].iter().all(|&t| t == 2 || t == 0));
    assert!(types.contains(&0), "SCSI commands go as type 0");
}
