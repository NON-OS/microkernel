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

//! The walk of the device list for NVMe controllers, in try order.

use nonos_libc::{mk_device_list, DeviceRecord};

use super::found::{Candidates, Found, MAX_CONTROLLERS};
use super::pci_match::{has_register_bar, is_nvme};
use super::rank;
use super::say_seen::say_seen;
use crate::constants::{CLASS_BLOCK, NVME_BAR_INDEX};

/// The device list holds ACPI and fabricated records beside PCI functions;
/// at 32 a machine with more stopped short of the device behind a root port.
const MAX_DEVICES: usize = 128;

/// Every NVMe controller in the device list, each said on the console, with
/// the ones that can be served ordered cache modules last. None when there
/// is none to serve.
pub fn find_nvme() -> Option<Candidates> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(CLASS_BLOCK, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let mut usable =
        [Found { device_id: 0, bar_size: 0, vendor: 0, device: 0, cache: false }; MAX_CONTROLLERS];
    let mut ids = [(0u16, 0u16); MAX_CONTROLLERS];
    let mut seen = 0usize;
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if !is_nvme(r) {
            continue;
        }
        let servable = has_register_bar(r);
        let cache = rank::is_cache_module(r.vendor, r.device);
        let kept = servable && seen < MAX_CONTROLLERS;
        say_seen(r, servable, cache, kept);
        if kept {
            let bar_size = r.bars[NVME_BAR_INDEX as usize].size;
            usable[seen] = Found {
                device_id: r.device_id,
                bar_size,
                vendor: r.vendor,
                device: r.device,
                cache,
            };
            ids[seen] = (r.vendor, r.device);
            seen += 1;
        }
    }
    if seen == 0 {
        return None;
    }
    let mut order = [0usize; MAX_CONTROLLERS];
    let count = rank::try_order(&ids[..seen], &mut order);
    let mut list = usable;
    for (slot, &i) in order[..count].iter().enumerate() {
        list[slot] = usable[i];
    }
    Some(Candidates { list, count })
}
