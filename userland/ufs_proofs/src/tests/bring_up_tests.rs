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

//! A whole bring-up against a QEMU-like host and device: the host enabled,
//! the link started once, the lists running, NOP OUT, fDeviceInit, and the
//! one logical unit served under its size and names.

use crate::model::{Cfg, Rig};
use crate::ufs::facts::hci::*;
use crate::ufs::facts::uic::DME_LINKSTARTUP;
use crate::ufs::facts::utp::TYPE_UFS_STORAGE;

#[test]
fn a_qemu_like_device_comes_up_with_its_unit() {
    let rig = Rig::new(Cfg::default());
    let ufs = rig.up(false).expect("the device comes up");
    let u = ufs.unit(0).expect("unit 0 served");
    assert_eq!((u.lun, u.blocks, u.block_len), (0, 1000, 4096));
    assert_eq!(&u.name, b"QEMU    QEMU HARDDISK   ");
    assert_eq!(&u.serial[..u.serial_len], b"ufs-lu-0");
    assert!(ufs.unit(1).is_none());
    assert_eq!(ufs.max_blocks(u), 16, "64 KiB of 4 KiB blocks");
    let h = rig.m.host.borrow();
    let starts = h.uic.iter().filter(|(c, _)| *c == DME_LINKSTARTUP).count();
    assert_eq!(starts, 1);
    assert_eq!(h.enables, 1);
    let l = rig.lists();
    assert_eq!(
        h.regs[(UTRLBA / 4) as usize] as u64 | (h.regs[(UTRLBAU / 4) as usize] as u64) << 32,
        l.utrl.bus
    );
    assert_eq!(h.regs[(UTMRLBA / 4) as usize] as u64, l.utmrl.bus & 0xFFFF_FFFF);
    assert_eq!((h.regs[(UTRLRSR / 4) as usize], h.regs[(UTMRLRSR / 4) as usize]), (RUN, RUN));
    assert!(h.utrds.iter().all(|(t, _)| *t == TYPE_UFS_STORAGE), "UFSHCI 4.0 takes type 1 for all");
    assert!(rig.m.said("driver.ufs: LU 0: 1000 x 4096 bytes, QEMU    QEMU HARDDISK"));
}
