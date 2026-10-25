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

//! READ, WRITE and SYNCHRONIZE CACHE on a served unit.

use crate::model::{Cfg, Lu, Rig};
use crate::ufs::disk::rw::{flush, read, write};

#[test]
fn what_is_written_reads_back_across_the_data_region() {
    let rig = Rig::new(Cfg::default());
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    let src: Vec<u8> = (0..16 * 4096).map(|i| (i * 7 % 251) as u8).collect();
    write(&rig.m, &ufs, &u, 984, 16, &src).expect("the last 16 blocks");
    let mut back = vec![0u8; 16 * 4096];
    read(&rig.m, &ufs, &u, 984, 16, &mut back).expect("read");
    assert_eq!(back, src);
    let mut one = vec![0xAAu8; 4096];
    read(&rig.m, &ufs, &u, 0, 1, &mut one).expect("an unwritten block");
    assert!(one.iter().all(|&b| b == 0));
    flush(&rig.m, &ufs, &u).expect("SYNCHRONIZE CACHE");
}

/// An LBA past 32 bits needs READ(16) and WRITE(16); the model reads the
/// LBA out of whichever CDB came, so the block lands where it was asked.
#[test]
fn blocks_past_32_bit_lbas_go_by_the_16_byte_commands() {
    let rig = Rig::new(Cfg { lus: vec![Lu::new(0, 512, 1 << 33)], ..Cfg::default() });
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    let lba = (1 << 32) + 7;
    write(&rig.m, &ufs, &u, lba, 1, &[0x5A; 512]).unwrap();
    assert_eq!(rig.m.cfg.borrow().lus[0].block(lba), vec![0x5A; 512]);
    let mut back = [0u8; 512];
    read(&rig.m, &ufs, &u, lba, 1, &mut back).unwrap();
    assert_eq!(back, [0x5A; 512]);
}
