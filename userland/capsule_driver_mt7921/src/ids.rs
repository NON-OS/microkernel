/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The PCI functions this driver takes, and the chip each one is.
//!
//! The table is Linux v6.12 `mt7921/pci.c`, `mt7921_pci_device_table`: the
//! MT7961 (MT7921E, AMD RZ608 as 0x0608), the MT7922 (AMD RZ616 as 0x0616,
//! and under the ITTIM vendor id), and the MT7920. The chip id the part
//! reports decides the firmware, as `mt792x_ram_name` does, not the PCI id.

pub const VENDOR_MEDIATEK: u16 = 0x14c3;
/// `PCI_VENDOR_ID_ITTIM` in Linux's `pci_ids.h`.
pub const VENDOR_ITTIM: u16 = 0x0b48;

pub const PCI_IDS: [(u16, u16); 6] = [
    (VENDOR_MEDIATEK, 0x7961),
    (VENDOR_MEDIATEK, 0x7922),
    (VENDOR_ITTIM, 0x7922),
    (VENDOR_MEDIATEK, 0x0608),
    (VENDOR_MEDIATEK, 0x0616),
    (VENDOR_MEDIATEK, 0x7920),
];

/// Whether this driver takes the function `vendor:device`.
pub fn supported(vendor: u16, device: u16) -> bool {
    PCI_IDS.contains(&(vendor, device))
}

/// The chip, from `MT_HW_CHIPID` and bit 7 of `MT_HW_BOUND`
/// (`mt7921_pci_probe`: a 0x7961 with that bit set is an MT7920).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chip {
    Mt7961,
    Mt7920,
    Mt7922,
}

impl Chip {
    pub fn from_ids(chipid: u32, bound: u32) -> Option<Chip> {
        match chipid & 0xffff {
            0x7961 if bound & (1 << 7) != 0 => Some(Chip::Mt7920),
            0x7961 => Some(Chip::Mt7961),
            0x7922 => Some(Chip::Mt7922),
            _ => None,
        }
    }

    /// The number serial lines name the chip by.
    pub fn number(self) -> u32 {
        match self {
            Chip::Mt7961 => 0x7961,
            Chip::Mt7920 => 0x7920,
            Chip::Mt7922 => 0x7922,
        }
    }
}
