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

extern crate alloc;

use alloc::vec::Vec;

use crate::drivers::pci::types::{HeaderType, PciDevice};
use crate::hardware::broker::DeviceRecord;

use super::pci_record::record_from_pci;
use super::state::{next_id, TABLE};
use crate::hardware::broker::claim::{reset_caps, say_reset_ways};
use crate::hardware::broker::pci_index::{self, PciHandle};

pub fn init_from_pci(devices: &[PciDevice]) {
    let mut records: Vec<DeviceRecord> = Vec::with_capacity(devices.len());
    let mut handles: Vec<PciHandle> = Vec::with_capacity(devices.len());
    for (idx, dev) in devices.iter().enumerate() {
        records.push(record_from_pci(idx as u64, dev));
        handles.push(handle_for(idx as u64, dev));
    }
    let ways: Vec<_> = handles.iter().map(|h| (h.address, h.reset)).collect();
    say_reset_ways(&ways);
    *TABLE.write() = records;
    pci_index::install(handles);
}

pub(in crate::hardware::broker) fn register_platform_device(mut record: DeviceRecord) -> u64 {
    let mut table = TABLE.write();
    let max = table.iter().map(|r| r.device_id).max();
    record.device_id = next_id(max, 0x1_0000_0000);
    table.push(record);
    record.device_id
}

/// The kernel-only handle for a PCI function, the same whether it was found
/// in the boot scan or added after it.
pub(super) fn handle_for(device_id: u64, dev: &PciDevice) -> PciHandle {
    PciHandle {
        device_id,
        address: dev.address,
        bars: dev.bars,
        msi: dev.msi,
        msix: dev.msix,
        // An Intel VMD gates its drives' DMA and config as a bridge does (L61).
        standard: matches!(dev.header_type, HeaderType::Standard)
            && !crate::hardware::inventory::is_intel_vmd(dev.vendor_id, dev.device_id),
        reset: reset_caps(dev),
    }
}
