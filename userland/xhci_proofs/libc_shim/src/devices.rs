// NONOS Operating System (AGPL-3.0-or-later)
//! MkDeviceList as the kernel answers it (src/syscall/microkernel/
//! device_list.rs): the records of the asked broker class, every record for
//! class 0, at most `count` of them copied, the number copied returned. The
//! machine's records are the test thread's own (`offer_devices`).

use std::cell::RefCell;

// Mounted whole, so the kinds and flags these tests do not name are unused.
#[allow(dead_code)]
#[path = "../../../libc/src/broker/types/bar.rs"]
mod bar;
#[allow(dead_code)]
#[path = "../../../libc/src/broker/types/bus.rs"]
mod bus;
#[allow(dead_code)]
#[path = "../../../libc/src/broker/types/device.rs"]
mod device;

pub use bar::{Bar, BAR_KIND_MMIO};
pub use bus::{BUS_KIND_FDT, BUS_KIND_PCI};
pub use device::DeviceRecord;

thread_local! {
    static MACHINE: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
}

/// The records this thread's machine lists, in bus order.
pub fn offer_devices(records: Vec<DeviceRecord>) {
    MACHINE.with(|m| *m.borrow_mut() = records);
}

pub fn mk_device_list(class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    MACHINE.with(|m| {
        let shown: Vec<DeviceRecord> =
            m.borrow().iter().filter(|r| class == 0 || r.class == class).copied().collect();
        if count == 0 {
            return shown.len() as i64;
        }
        let n = shown.len().min(count as usize);
        // SAFETY: the caller hands a buffer of `count` records, as the
        // kernel's validate_user_write requires of a real one.
        let out = unsafe { std::slice::from_raw_parts_mut(buf, n) };
        out.copy_from_slice(&shown[..n]);
        n as i64
    })
}
