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

//! Reads and writes of an SD card through the real disk surface: no
//! SET_BLOCK_COUNT, so a multi-block transfer is ended by the host's own
//! CMD12, in the card's own addressing.

use crate::emmc::disk::SECTOR_SIZE;
use crate::emmc::error::EmmcError;
use crate::model::sd::{sdsc, SdCfg};
use crate::model::{sd_slot, Rig, TestDisk};

fn up(cfg: SdCfg) -> (Rig, TestDisk) {
    let rig = Rig::sd(sd_slot(), cfg);
    let disk = rig.disk(false).expect("bring-up");
    (rig, disk)
}

#[test]
fn writes_read_back_on_both_kinds_of_card() {
    for cfg in [SdCfg::default(), sdsc()] {
        let (rig, mut disk) = up(cfg);
        let last = disk.capacity_sectors() - 64;
        for (lba, n) in [(0u64, 1u32), (9, 8), (1_000, 64), (last, 64)] {
            let data: Vec<u8> =
                (0..n as usize * SECTOR_SIZE).map(|i| (i as u8) ^ (lba as u8)).collect();
            disk.write(lba, n, &data).expect("write");
            let mut back = vec![0u8; data.len()];
            disk.read(lba, n, &mut back).expect("read");
            assert_eq!(back, data, "lba {lba} n {n}");
            let sim = rig.sim.borrow();
            let store = &sim.sd.as_ref().expect("an SD card").store;
            assert_eq!(&store[&(lba + n as u64 - 1)][..], &data[data.len() - SECTOR_SIZE..]);
        }
        assert!(!rig.indices().contains(&23), "no SET_BLOCK_COUNT");
        rig.assert_clean();
    }
}

#[test]
fn nothing_past_the_card_is_asked_for() {
    let (rig, mut disk) = up(SdCfg::default());
    let before = rig.cmds().len();
    let end = disk.capacity_sectors();
    let mut buf = vec![0u8; 2 * SECTOR_SIZE];
    assert_eq!(disk.read(end - 1, 2, &mut buf), Err(EmmcError::OutOfRange));
    assert_eq!(rig.cmds().len(), before);
}
