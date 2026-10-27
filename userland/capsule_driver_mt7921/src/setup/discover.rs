/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The first PCI function of the table in `ids`, from the broker's device
//! list, with its BAR0, which must be memory (`pcim_iomap_regions(pdev,
//! BIT(0))` in `mt7921_pci_probe`).

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::ids::supported;

const MAX_DEVICES: usize = 96;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub bar_size: u64,
}

pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let n = usize::try_from(n).ok()?.min(MAX_DEVICES);
    buf[..n].iter().find_map(|r| {
        let bar = r.bars.first().filter(|b| r.bar_count > 0 && b.kind == BAR_KIND_MMIO)?;
        let ours = r.bus_kind == BUS_KIND_PCI && supported(r.vendor, r.device);
        (ours && bar.size >= 0x100000)
            .then_some(Found { device_id: r.device_id, bar_size: bar.size })
    })
}
