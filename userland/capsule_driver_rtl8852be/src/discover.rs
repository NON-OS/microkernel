// NONOS Operating System (AGPL-3.0-or-later)

//! Find the RTL8852BE on the PCI bus through the broker's device list. The
//! ids are Linux v6.12 `rtw89/rtw8852be.c`'s `rtw89_8852be_id_table`
//! (10ec:b852, 10ec:b85b). The registers are in a memory BAR that is not
//! BAR0 (`pci.c`, `rtw89_pci_setup_mapping`, maps bar_id 2), so the first
//! memory BAR is taken by scanning, and one too small for the highest
//! register the driver writes is refused.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

pub const PCI_VENDOR_REALTEK: u16 = 0x10ec;
/// The RTL8852BE's device ids.
pub const RTL8852BE_DEVICE_IDS: [u16; 2] = [0xb852, 0xb85b];
/// The smallest register window the driver can use: the indirect access
/// window (`R_AX_INDIR_ACCESS_ENTRY`, `mac::macid`) sits at 0x40000, past
/// the PHY's at 0x10000. A BAR's size is a power of two, so that is 512 KiB
/// (the card's BAR is 1 MiB).
pub const BAR_MIN: u64 = 0x8_0000;
const MAX_DEVICES: usize = 96;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub bar_index: u32,
    pub bar_size: u64,
}

/// The chip, or `None` when the board carries no RTL8852BE.
pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..(n as usize).min(MAX_DEVICES)].iter().find_map(pick)
}

/// The record's memory BAR, when it is an RTL8852BE with a window big enough.
pub fn pick(r: &DeviceRecord) -> Option<Found> {
    if r.bus_kind != BUS_KIND_PCI
        || r.vendor != PCI_VENDOR_REALTEK
        || !RTL8852BE_DEVICE_IDS.contains(&r.device)
    {
        return None;
    }
    let count = (r.bar_count as usize).min(r.bars.len());
    let (i, bar) = r.bars[..count].iter().enumerate().find(|(_, b)| b.kind == BAR_KIND_MMIO)?;
    (bar.size >= BAR_MIN).then_some(Found {
        device_id: r.device_id,
        bar_index: i as u32,
        bar_size: bar.size,
    })
}
