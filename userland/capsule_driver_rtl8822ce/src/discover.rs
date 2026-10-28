// NONOS Operating System (AGPL-3.0-or-later)

//! Find the RTL8822CE on the PCI bus through the broker's device list: the
//! Realtek vendor, an 8822CE device id, and its first memory BAR. rtw88 maps
//! bar_id 2 (BAR0 is the I/O BAR), so the MMIO BAR is found, not assumed.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::{PCI_VENDOR_REALTEK, RTL8822CE_DEVICE_IDS};

const MAX_DEVICES: usize = 96;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    /// The memory BAR the registers are in, found by its kind.
    pub bar_index: u32,
    pub bar_size: u64,
}

/// Locate the wifi chip, or `None` when the board carries a different adapter.
pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    pick(&buf[..core::cmp::min(n as usize, MAX_DEVICES)])
}

/// The first 8822CE in `records` with a sized memory BAR.
pub fn pick(records: &[DeviceRecord]) -> Option<Found> {
    for r in records {
        if r.bus_kind != BUS_KIND_PCI
            || r.vendor != PCI_VENDOR_REALTEK
            || !RTL8822CE_DEVICE_IDS.contains(&r.device)
        {
            continue;
        }
        let count = core::cmp::min(r.bar_count as usize, r.bars.len());
        for (i, bar) in r.bars[..count].iter().enumerate() {
            if bar.kind == BAR_KIND_MMIO && bar.size != 0 {
                return Some(Found {
                    device_id: r.device_id,
                    bar_index: i as u32,
                    bar_size: bar.size,
                });
            }
        }
    }
    None
}
