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
//! The functions that could be an LPSS SPI host: Intel, of the classes the
//! LPSS functions carry (serial bus or signal processing, subclass other),
//! with a memory BAR 0 large enough for the private block. Which of them is
//! SPI is proved after the claim, by the LPSS capabilities register, and
//! the functions driver.i2c_pci0 lists as I2C are never tried; the
//! firmware's flash controller is refused by the broker before that, by
//! where it sits, so it is never claimed here (broker/firmware_flash.rs).

use alloc::vec::Vec;

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO};

const INTEL: u16 = 0x8086;
const CLASSES: [(u8, u8); 2] = [(0x0C, 0x80), (0x11, 0x80)];
/// The private block ends at 0x300; LPSS BARs are 4 KiB.
const BAR_MIN: u64 = 0x1000;
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy, Debug)]
pub struct Found {
    pub device_id: u64,
    pub device: u16,
    pub bar_size: u64,
}

pub fn candidates() -> Option<Vec<Found>> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let n = usize::try_from(n).ok()?.min(MAX_DEVICES);
    let found: Vec<Found> = buf[..n]
        .iter()
        .filter(|r| is_candidate(r))
        .map(|r| Found { device_id: r.device_id, device: r.device, bar_size: r.bars[0].size })
        .collect();
    (!found.is_empty()).then_some(found)
}

pub fn is_candidate(r: &DeviceRecord) -> bool {
    let bar = r.bars[0];
    r.vendor == INTEL
        && crate::i2c_ids::device_info(r.device).is_none()
        && CLASSES.contains(&(r.pci_class, r.pci_subclass))
        && r.bar_count > 0
        && bar.kind == BAR_KIND_MMIO
        && bar.size >= BAR_MIN
}
