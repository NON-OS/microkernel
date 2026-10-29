// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Which chip Read Local Version names (Vol 4, Part E, 7.4.1). btrtl.c's
//! table has the RTL8821C on USB as LMP subversion 0x8821, HCI revision
//! 0x000c and HCI version 0x08 ("8821C", IC_INFO(RTL_ROM_LMP_8821A, 0xc,
//! 0x8, HCI_USB)). A patched controller reports the patch's version in
//! those fields instead, so it no longer matches; btrtl.c then assumes no
//! upload is needed, and so does this host.

use nonos_bt_hci::HciError;
use nonos_bt_wire::cursor::Reader;

use crate::error::RtlError;

pub const LMP_8821: u16 = 0x8821;
pub const HCI_REV_8821C: u16 = 0x000c;
pub const HCI_VER_8821C: u8 = 0x08;
/// Realtek's company identifier (Bluetooth SIG Assigned Numbers).
pub const REALTEK_COMPANY: u16 = 0x005d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version {
    pub hci_ver: u8,
    pub hci_rev: u16,
    pub lmp_ver: u8,
    pub company: u16,
    pub subversion: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chip {
    /// The RTL8821C's ROM, waiting for its patch.
    Rom8821c,
    /// A Realtek controller that is not an unpatched RTL8821C: already
    /// running a patch, or a chip this host has no patch for.
    Realtek,
    /// Another company's controller.
    Other,
}

/// Read Local Version's return parameters, the status already taken off.
pub fn version(ret: &[u8]) -> Result<Version, RtlError> {
    let wire = |e| RtlError::Hci(HciError::Wire(e));
    let mut r = Reader::new(ret);
    Ok(Version {
        hci_ver: r.u8().map_err(wire)?,
        hci_rev: r.u16().map_err(wire)?,
        lmp_ver: r.u8().map_err(wire)?,
        company: r.u16().map_err(wire)?,
        subversion: r.u16().map_err(wire)?,
    })
}

pub fn chip(v: &Version) -> Chip {
    let rom = (v.subversion, v.hci_rev, v.hci_ver) == (LMP_8821, HCI_REV_8821C, HCI_VER_8821C);
    match (rom, v.company == REALTEK_COMPANY) {
        (true, _) => Chip::Rom8821c,
        (false, true) => Chip::Realtek,
        (false, false) => Chip::Other,
    }
}
