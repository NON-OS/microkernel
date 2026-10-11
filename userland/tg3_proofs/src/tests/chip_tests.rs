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

//! Which functions are driven: the ten 57765-class IDs of tg3_pci_tbl, the
//! revision read where tg3_detect_asic_rev reads it, anything not of the
//! class refused before a register is written, and host control set first.

use nonos_libc::{config_set, config_writes};

use crate::chip::identify;
use crate::constants::chip::*;
use crate::constants::pci::{part_of, TG3_PARTS};
use crate::constants::pcicfg::*;
use crate::regs::Regs;
use crate::setup::config::Config;

use super::model::window;

#[test]
fn the_ten_parts_and_their_10_100_flags() {
    let ids: Vec<u16> = TG3_PARTS.iter().map(|(id, _)| *id).collect();
    let want = [0x16B1, 0x16B5, 0x16B0, 0x16B4, 0x16B2, 0x16B6, 0x1682, 0x1686, 0x16B7, 0x16B3];
    assert_eq!(ids, want);
    assert_eq!(part_of(0x16B2), Some(true));
    assert_eq!(part_of(0x16B6), Some(true));
    assert_eq!(part_of(0x16B4), Some(false));
    // The 5761, 57780, 5717 and 5762 are other classes.
    for other in [0x1681, 0x1692, 0x1655, 0x1687, 0x0000] {
        assert_eq!(part_of(other), None, "{other:#x}");
    }
}

#[test]
fn the_revision_comes_from_the_product_id_register() {
    assert!(is_57765_class(0x5778_5100) && is_57765_class(0x5776_6000));
    assert!(!is_57765_class(0x5778_0001) && !is_57765_class(0x0571_7000));
    let bar = window();
    let cfg = Config { device_id: 9, epoch: 1 };
    config_set(TG3PCI_MISC_HOST_CTRL, 0xF000_0000 | 0x0C00, 4);
    config_set(PCI_CAPABILITY_LIST, 0x48, 1);
    config_set(0x48, 0x0010, 2);
    config_set(TG3PCI_GEN15_PRODID_ASICREV, 0x5776_6001, 4);
    let (chip, cap) = identify(&cfg, &Regs::new(bar.base())).expect("57766");
    assert_eq!((chip.rev, cap), (0x5776_6001, 0x48));
    // The bits above the control field are kept; the field is tg3's word.
    assert_eq!(config_writes(), [(TG3PCI_MISC_HOST_CTRL, 0x0C9A)]);
}

#[test]
fn another_class_is_refused_before_anything_is_written() {
    let bar = window();
    config_set(TG3PCI_MISC_HOST_CTRL, 0xF000_0000, 4);
    config_set(TG3PCI_GEN15_PRODID_ASICREV, 0x0571_9000, 4);
    let cfg = Config { device_id: 9, epoch: 1 };
    assert!(identify(&cfg, &Regs::new(bar.base())).is_err());
    assert!(config_writes().is_empty());
    assert!((0..bar.len()).step_by(4).all(|at| bar.wrote32(at) == 0));
}
