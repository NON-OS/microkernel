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

//! Whether one vendor capability names a range the driver can map and use.

use super::layout::{CFG_COMMON, CFG_DEVICE, CFG_NOTIFY, COMMON_CFG_LEN};
use super::msix::MsixLayout;
use super::region::Region;
use crate::pci::Bars;

/// The region must sit in a present memory BAR, be long enough for the
/// registers read from it, start on the alignment the specification gives
/// its structure, keep every page a mapping of it covers inside the BAR
/// (the broker maps whole pages and refuses past the BAR's end), and stay
/// clear of the MSI-X table and PBA pages.
pub(super) fn usable(cfg_type: u8, region: Region, bars: &Bars, msix: Option<&MsixLayout>) -> bool {
    let Some(bar) = bars.get(region.bar as usize) else {
        return false;
    };
    if !bar.is_mmio() {
        return false;
    }
    if region.length < min_len(cfg_type) || !region.offset.is_multiple_of(align(cfg_type)) {
        return false;
    }
    let (start, end) = region.page_span();
    if end > bar.size {
        return false;
    }
    !msix.is_some_and(|m| m.blocks(region.bar, start, end))
}

/// The bytes each structure must hold: the whole common configuration, one
/// 16-bit notify write, one byte otherwise.
fn min_len(cfg_type: u8) -> u32 {
    match cfg_type {
        CFG_COMMON => COMMON_CFG_LEN,
        CFG_NOTIFY => 2,
        _ => 1,
    }
}

/// Common and device configuration are read 32 bits at a time, so they must
/// be 4-byte aligned (the specification requires it of the device
/// configuration); notify writes are 16 bits and the specification requires
/// 2-byte alignment; the ISR is one byte.
fn align(cfg_type: u8) -> u32 {
    match cfg_type {
        CFG_COMMON | CFG_DEVICE => 4,
        CFG_NOTIFY => 2,
        _ => 1,
    }
}
