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

//! The capability parse. The first usable capability of each structure type
//! wins, which is what the virtio specification asks of a driver ("use the
//! first instance of each virtio structure type they can support"). An
//! unusable one (short, in an I/O or absent BAR, past the BAR's end, over
//! the MSI-X table) is skipped and a later one of the same type can still
//! be taken; QEMU's optional port-I/O notify capability is one such.

use super::layout::{
    CAP_BAR, CAP_CFG_TYPE, CAP_ID_MSIX, CAP_ID_VENDOR, CAP_LEN, CAP_LENGTH, CAP_NOTIFY_MULTIPLIER,
    CAP_OFFSET, CFG_COMMON, CFG_DEVICE, CFG_ISR, CFG_NOTIFY, NOTIFY_CAP_MIN_LEN,
    VIRTIO_CAP_MIN_LEN,
};
use super::msix::MsixLayout;
use super::region::Region;
use super::types::ModernCaps;
use super::validate::usable;
use super::walk::CapWalk;
use crate::pci::{Bars, ConfigSpace, CONFIG_SPACE_LEN};

struct VendorCap {
    cfg_type: u8,
    region: Region,
    multiplier: u32,
}

pub fn parse(cfg: &ConfigSpace, bars: &Bars) -> ModernCaps {
    let mut caps = ModernCaps {
        msix: CapWalk::new(cfg)
            .find(|&ptr| cfg.u8_at(ptr) == Some(CAP_ID_MSIX))
            .and_then(|ptr| MsixLayout::read(cfg, ptr)),
        ..ModernCaps::default()
    };
    for ptr in CapWalk::new(cfg) {
        if cfg.u8_at(ptr) != Some(CAP_ID_VENDOR) {
            continue;
        }
        let Some(cap) = vendor_cap(cfg, ptr) else {
            continue;
        };
        if !usable(cap.cfg_type, cap.region, bars, caps.msix.as_ref()) {
            continue;
        }
        match cap.cfg_type {
            CFG_COMMON => keep_first(&mut caps.common, cap.region),
            CFG_ISR => keep_first(&mut caps.isr, cap.region),
            CFG_DEVICE => keep_first(&mut caps.device, cap.region),
            CFG_NOTIFY if caps.notify.is_none() => {
                caps.notify = Some(cap.region);
                caps.notify_multiplier = cap.multiplier;
            }
            _ => {}
        }
    }
    caps
}

fn keep_first(slot: &mut Option<Region>, region: Region) {
    if slot.is_none() {
        *slot = Some(region);
    }
}

/// The fields of one vendor capability, read only as far as its own
/// cap_len covers: a capability too short for a field it would need, or
/// whose length runs past config space, is not read at all.
fn vendor_cap(cfg: &ConfigSpace, ptr: usize) -> Option<VendorCap> {
    let cap_len = cfg.u8_at(ptr + CAP_LEN)?;
    if cap_len < VIRTIO_CAP_MIN_LEN || ptr + cap_len as usize > CONFIG_SPACE_LEN {
        return None;
    }
    let cfg_type = cfg.u8_at(ptr + CAP_CFG_TYPE)?;
    let region = Region {
        bar: cfg.u8_at(ptr + CAP_BAR)?,
        offset: cfg.u32_at(ptr + CAP_OFFSET)?,
        length: cfg.u32_at(ptr + CAP_LENGTH)?,
    };
    let multiplier = if cfg_type == CFG_NOTIFY {
        if cap_len < NOTIFY_CAP_MIN_LEN {
            return None;
        }
        let m = cfg.u32_at(ptr + CAP_NOTIFY_MULTIPLIER)?;
        if !multiplier_ok(m) {
            return None;
        }
        m
    } else {
        0
    };
    Some(VendorCap { cfg_type, region, multiplier })
}

/// The specification allows zero (one shared address) or an even power of
/// two. Anything else would put a 16-bit notify write on an odd address.
fn multiplier_ok(m: u32) -> bool {
    m == 0 || (m.is_power_of_two() && m >= 2)
}
