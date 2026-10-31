// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The whole load against the modelled ROM: version, ROM version, every
//! fragment, and a controller that then runs the patch.

use nonos_bt_rtl::chip::{chip as which, version, Chip};
use nonos_bt_rtl::{prepare, Outcome};
use nonos_bt_wire::hci::opcode::READ_LOCAL_VERSION;

use crate::firmware::{CONFIG, FW};
use crate::fixture::{chip, expected, rom, FW_VERSION, HP};

#[test]
fn the_hp_card_is_patched_for_its_rom() {
    for r in [0, 1] {
        let mut h = rom(r);
        let out = prepare(&mut h, HP, FW, CONFIG).unwrap();
        let n = expected(r).len() / 252 + 1;
        assert_eq!(out, Outcome::Patched { rom: r, fw_version: FW_VERSION, fragments: n });
        let m = chip(&mut h);
        assert_eq!((m.broken, m.running), (None, Some(FW_VERSION)), "ROM {r}");
        assert_eq!(m.got, expected(r));
        let mut want = vec![0x1001, 0xfc6d];
        want.extend(std::iter::repeat_n(0xfc20, n));
        assert_eq!(m.opcodes(), want);
    }
}

#[test]
fn every_rtl8821c_id_btusb_lists_is_patched() {
    for ids in [(0x0bda, 0xb00a), (0x0bda, 0xc821), (0x13d3, 0x3529)] {
        let mut h = rom(1);
        assert!(
            matches!(prepare(&mut h, ids, FW, CONFIG), Ok(Outcome::Patched { .. })),
            "{ids:x?}"
        );
    }
}

#[test]
fn once_patched_the_controller_names_the_patch_not_the_rom() {
    let mut h = rom(1);
    prepare(&mut h, HP, FW, CONFIG).unwrap();
    let v = version(&h.command(READ_LOCAL_VERSION, &[]).unwrap()).unwrap();
    assert_eq!((v.hci_rev, v.subversion), (0x75b8, 0xf098));
    assert_eq!(which(&v), Chip::Realtek);
}

#[test]
fn a_controller_bound_again_is_not_patched_twice() {
    let mut h = rom(1);
    prepare(&mut h, HP, FW, CONFIG).unwrap();
    let before = chip(&mut h).sent.len();
    assert_eq!(prepare(&mut h, HP, FW, CONFIG), Ok(Outcome::Running));
    assert_eq!(chip(&mut h).opcodes()[before..], [0x1001], "a vendor command went out");
}
