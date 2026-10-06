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

//! The order controllers are tried in. A laptop may carry an Intel Optane
//! memory module beside its SSD: a small NVMe function the firmware's RAID
//! driver pairs with the disk as a cache, holding no file system of its own.
//! Taking it first would serve a 16 or 32 GB cache and leave the disk unused,
//! so it is tried after every other controller. Pure, so the order is held
//! on the host.

const VENDOR_INTEL: u16 = 0x8086;

/// Intel Optane memory cache functions (the Optane Memory series, 16 and
/// 32 GB M.2 modules). Optane SSDs proper (900P, 905P, DC P4800X) are disks
/// and are not listed. Another cache id joins this list when one is seen.
const CACHE_ONLY: [u16; 1] = [0x2522];

/// Whether `vendor:device` is an NVMe function that only caches another
/// disk.
pub const fn is_cache_module(vendor: u16, device: u16) -> bool {
    if vendor != VENDOR_INTEL {
        return false;
    }
    let mut i = 0;
    while i < CACHE_ONLY.len() {
        if CACHE_ONLY[i] == device {
            return true;
        }
        i += 1;
    }
    false
}

/// Fill `order` with indices into `ids` (vendor, device): every controller
/// that is not a cache module first, in the device list's order, then the
/// cache modules. Returns how many indices were written.
pub fn try_order(ids: &[(u16, u16)], order: &mut [usize]) -> usize {
    let mut n = 0usize;
    for cache_pass in [false, true] {
        for (i, &(vendor, device)) in ids.iter().enumerate() {
            if is_cache_module(vendor, device) == cache_pass && n < order.len() {
                order[n] = i;
                n += 1;
            }
        }
    }
    n
}
