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

//! The functions behind every VMD, as device records the broker can list.

extern crate alloc;

use alloc::vec::Vec;

use super::super::config::ConfigSpace;
use super::super::types::{PciAddress, PciDevice};
use super::ensure::ensure;
use super::probe::probe;
use super::registry::DOMAINS;
use super::report::child;

/// Bring the VMD domains up if this is the first call, then list the
/// endpoints behind them. Bridges are left out: the broker has nothing to
/// hand a capsule for one.
pub fn children(segment0: &[PciDevice]) -> Vec<PciDevice> {
    ensure(segment0);
    let mut out = Vec::new();
    let Some(domains) = DOMAINS.get() else {
        return out;
    };
    for d in domains.iter() {
        let last = (d.bus_start as u16 + d.bus_count - 1).min(255) as u8;
        for bus in d.bus_start..=last {
            for device in 0..32u8 {
                let first = ConfigSpace::new(PciAddress::in_segment(d.segment, bus, device, 0));
                let Ok(vendor) = first.vendor_id() else { continue };
                if vendor == 0xFFFF || vendor == 0 {
                    continue;
                }
                let multi = first.read8(0x0E).map(|h| h & 0x80 != 0).unwrap_or(false);
                for function in 0..if multi { 8 } else { 1 } {
                    let address = PciAddress::in_segment(d.segment, bus, device, function);
                    if let Some(dev) = probe(address) {
                        child(&dev);
                        out.push(dev);
                    }
                }
            }
        }
    }
    out
}
