// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! A controller that already runs a patch, left by another system or an
//! earlier bind, gets no vendor command: Read Local Version no longer names
//! the ROM. Nor does a chip that is not the RTL8821C's ROM.

use nonos_bt_rtl::chip::{chip as which, Chip, Version};
use nonos_bt_rtl::{prepare, Outcome};

use crate::firmware::{CONFIG, FW};
use crate::fixture::{chip, rom, HP};

#[test]
fn a_controller_running_a_patch_is_left_as_it_is() {
    let mut h = rom(1);
    chip(&mut h).running = Some(0x1234_5678);
    assert_eq!(prepare(&mut h, HP, FW, CONFIG), Ok(Outcome::Running));
    assert_eq!(chip(&mut h).opcodes(), [0x1001]);
}

fn v(hci_ver: u8, hci_rev: u16, company: u16, subversion: u16) -> Version {
    Version { hci_ver, hci_rev, lmp_ver: hci_ver, company, subversion }
}

#[test]
fn only_the_rtl8821c_rom_is_taken_for_one() {
    assert_eq!(which(&v(8, 0x000c, 0x005d, 0x8821)), Chip::Rom8821c);
    // The RTL8821A's ROM: the same subversion, another revision and version.
    assert_eq!(which(&v(6, 0x000a, 0x005d, 0x8821)), Chip::Realtek);
    // The RTL8822C's ROM, which this host has no patch for.
    assert_eq!(which(&v(0x0a, 0x000c, 0x005d, 0x8822)), Chip::Realtek);
    assert_eq!(which(&v(8, 0x000c, 0x0002, 0x1234)), Chip::Other);
}
