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

//! An SD card in a removable slot, brought up by the real driver against
//! the model card: the commands in the order the SD specification has
//! them, the address the card chose, four lines at Default Speed.

use crate::model::sd::{sdsc, SdCfg};
use crate::model::{sd_slot, Rig, St};

#[test]
fn a_high_capacity_card_comes_up_on_four_lines() {
    let rig = Rig::sd(sd_slot(), SdCfg::default());
    let disk = rig.disk(false).expect("bring-up");
    let c = disk.card;
    assert!(c.sd && c.sector_mode && !c.cmd23 && c.ext.is_none());
    assert_eq!((c.rca, c.width, c.hz), (0xb368, 4, 25_000_000));
    assert_eq!(c.sectors, (0xe4df + 1) * 1024);
    #[rustfmt::skip]
    let want = [0, 8, 55, 41, 55, 41, 55, 41, 55, 41, 2, 3, 9, 7, 55, 6, 13, 17];
    assert_eq!(rig.indices(), want);
    let sim = rig.sim.borrow();
    let card = sim.sd.as_ref().expect("an SD card");
    assert_eq!((card.state, card.width), (St::Tran, 4));
    // The query, then the 3.2-3.4 V window with HCS.
    assert_eq!(card.acmd41_args, vec![0, 0x4030_0000, 0x4030_0000, 0x4030_0000]);
    assert_eq!(sim.power_ons, vec![7], "one power up, at 3.3 V");
    drop(sim);
    let n = disk.names();
    assert_eq!((n.model(), &n.serial[..]), (&b"SD SL32G"[..], &b"1234ABCD"[..]));
    rig.assert_clean();
}

#[test]
fn a_version_one_card_is_addressed_in_bytes_with_its_block_length_set() {
    let rig = Rig::sd(sd_slot(), sdsc());
    let disk = rig.disk(false).expect("bring-up");
    let c = disk.card;
    assert!(c.sd && !c.sector_mode);
    assert_eq!(c.sectors, ((0xf1f + 1) << 9 << 10) / 512);
    let i = rig.indices();
    assert_eq!(&i[..4], &[0, 8, 55, 41], "CMD8 unanswered, then ACMD41");
    assert!(i.contains(&16), "SET_BLOCKLEN for a byte-mode card");
    let sim = rig.sim.borrow();
    let args = &sim.sd.as_ref().expect("an SD card").acmd41_args;
    assert!(args[1..].iter().all(|&a| a == 0x0030_0000), "no HCS for version 1: {args:x?}");
    drop(sim);
    rig.assert_clean();
}
