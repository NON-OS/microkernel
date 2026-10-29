/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Which chip, from what it says: its id, its ROM version's low octet and
//! its flavour bit, as Linux v6.12 `btmtk_usb_setup` reads them and
//! `btmtk_fw_get_filename` names a patch from them. linux-firmware ships
//! two this driver takes: `BT_RAM_CODE_MT7961_1_2_hdr.bin` (MT7921: id
//! 0x7961, flavour 0, version octet 1) and `BT_RAM_CODE_MT7922_1_1_hdr.bin`
//! (id 0x7922, version octet 0). Anything else names a file not in the
//! package, and is refused before the chip is touched.

use super::fail::Fail;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chip {
    Mt7961,
    Mt7922,
}

impl Chip {
    pub fn name(self) -> &'static [u8] {
        match self {
            Chip::Mt7961 => b"MT7921",
            Chip::Mt7922 => b"MT7922",
        }
    }
}

/// The flavour is bit 7 of the flavour register.
pub fn chip(id: u32, version: u32, flavour: u32) -> Result<Chip, Fail> {
    let (one_a, patch) = (flavour >> 7 & 1 == 1, (version & 0xff) + 1);
    match (id, one_a, patch) {
        (0x7961, false, 2) => Ok(Chip::Mt7961),
        (0x7922, _, 1) => Ok(Chip::Mt7922),
        _ => Err(Fail::Chip),
    }
}
