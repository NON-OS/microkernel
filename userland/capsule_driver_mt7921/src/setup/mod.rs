/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Finding the card, claiming it and mapping BAR0 through the broker, and
//! its PCI command register: Memory Space at the claim, Bus Master only
//! once the card is quiet and its rings are about to be given to it
//! (`pci_set_master` in `mt7921_pci_probe`).

mod discover;

pub use discover::{find, Found};

use nonos_libc::{
    mk_device_claim, mk_device_release, mk_mmio_map, mk_pci_config_write, MmioMapOut,
    MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

/// A claimed card: its broker identity and its BAR0 window.
pub struct Mapped {
    pub device_id: u64,
    pub epoch: u64,
    pub va: u64,
    pub len: u64,
}

/// The command register: Memory Space, and Bus Master when `master`.
pub fn command(device_id: u64, epoch: u64, master: bool) -> bool {
    let bus = if master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    mk_pci_config_write(device_id, epoch, MK_PCI_CFG_COMMAND, MK_PCI_CMD_MEMORY_SPACE | bus) >= 0
}

/// Claim `f`, turn its memory decoding on and map BAR0. A failure after
/// the claim gives the claim back, so a later attempt can take it.
pub fn claim(f: Found) -> Result<Mapped, &'static str> {
    let epoch = mk_device_claim(f.device_id);
    if epoch <= 0 {
        return Err("mt7921: claim refused");
    }
    let epoch = epoch as u64;
    if !command(f.device_id, epoch, false) {
        let _ = mk_device_release(f.device_id);
        return Err("mt7921: memory space refused");
    }
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    if mk_mmio_map(f.device_id, epoch, 0, 0, 0, f.bar_size, &mut out) < 0 {
        let _ = mk_device_release(f.device_id);
        return Err("mt7921: BAR0 map refused");
    }
    Ok(Mapped { device_id: f.device_id, epoch, va: out.user_va, len: out.length })
}
