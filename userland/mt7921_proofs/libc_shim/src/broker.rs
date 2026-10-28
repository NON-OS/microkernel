// NONOS Operating System (AGPL-3.0-or-later)

//! The device broker as `setup` calls it: a device list a proof sets, a
//! claim that succeeds, BAR0 never mapped (the proofs build the chip's
//! window from the model), and the command register's writes recorded.

use std::cell::RefCell;

pub use crate::types::bar::{Bar, BAR_KIND_MMIO};
pub use crate::types::bus::BUS_KIND_PCI;
pub use crate::types::device::DeviceRecord;
pub use crate::types::dma::DmaMapOut;
pub use crate::types::mmio::MmioMapOut;

pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;

thread_local! {
    static DEVICES: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
    static COMMANDS: RefCell<Vec<u16>> = const { RefCell::new(Vec::new()) };
}

/// The broker's device list from now on.
pub fn set_devices(list: Vec<DeviceRecord>) {
    DEVICES.with(|d| *d.borrow_mut() = list);
}

/// Every command-register value written, oldest first.
pub fn commands() -> Vec<u16> {
    COMMANDS.with(|c| c.borrow().clone())
}

pub fn mk_device_list(_class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    DEVICES.with(|d| {
        let d = d.borrow();
        let n = d.len().min(count as usize);
        for (i, r) in d.iter().take(n).enumerate() {
            // SAFETY: the driver passes `count` records of room.
            unsafe { *buf.add(i) = *r };
        }
        n as i64
    })
}

pub fn mk_device_claim(_device_id: u64) -> i64 {
    1
}

pub fn mk_device_release(_device_id: u64) -> i64 {
    0
}

pub fn mk_mmio_map(
    _d: u64,
    _e: u64,
    _b: u32,
    _f: u32,
    _o: u64,
    len: u64,
    out: *mut MmioMapOut,
) -> i64 {
    // SAFETY: the driver passes its own out-record.
    unsafe { *out = MmioMapOut { user_va: 0, length: len, grant_id: 1 } };
    0
}

pub fn mk_pci_config_write(_d: u64, _e: u64, _offset: u32, value: u16) -> i64 {
    COMMANDS.with(|c| c.borrow_mut().push(value));
    0
}
