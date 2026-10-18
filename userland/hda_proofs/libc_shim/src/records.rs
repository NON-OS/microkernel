// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The device list discovery reads, as a test offers it.

use std::cell::RefCell;

pub const BUS_KIND_PCI: u8 = 1;
pub const BAR_KIND_MMIO: u8 = 1;

#[derive(Clone, Copy, Default)]
pub struct Bar {
    pub size: u64,
    pub kind: u8,
}

#[derive(Clone, Copy, Default)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub pci_class: u8,
    pub pci_subclass: u8,
    pub class: u32,
    pub vendor: u16,
    pub device: u16,
    pub bar_count: u8,
    pub irq_line: u8,
    pub irq_pin: u8,
    pub bars: [Bar; 6],
}

impl DeviceRecord {
    pub fn empty() -> Self {
        Self::default()
    }
}

thread_local! {
    static OFFERED: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
}

/// The records the device list answers with on this thread.
pub fn offer_devices(records: Vec<DeviceRecord>) {
    OFFERED.with(|o| *o.borrow_mut() = records);
}

/// Every offered record of `class`, or of every class for 0, as the broker
/// answers: at most `count` of them written to `buf`.
pub fn mk_device_list(class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    let offered = OFFERED.with(|o| o.borrow().clone());
    let fit: Vec<DeviceRecord> =
        offered.into_iter().filter(|r| class == 0 || r.class == class).collect();
    let n = fit.len().min(count as usize);
    // SAFETY: the caller hands a buffer of `count` records, as the real call
    // requires, and at most that many are written.
    unsafe { std::ptr::copy_nonoverlapping(fit.as_ptr(), buf, n) };
    n as i64
}
