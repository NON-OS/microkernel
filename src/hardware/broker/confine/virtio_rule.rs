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

//! Whether a virtio device may be confined. Pure, and
//! proven on the host (kernel_proofs::virtio_rule). A virtio device that
//! does not offer VIRTIO_F_ACCESS_PLATFORM (bit 33) DMAs to guest-physical
//! addresses whatever the IOMMU says, so under a unit in service it is
//! refused, as is every transitional ID, whose legacy path cannot offer it.

pub(crate) const VIRTIO_VENDOR: u16 = 0x1AF4;
const TRANSITIONAL: core::ops::RangeInclusive<u16> = 0x1000..=0x103F;
const MODERN: core::ops::RangeInclusive<u16> = 0x1040..=0x107F;
/// Capability entries walked at most; a longer or looping list is refused.
const MAX_CAPS: usize = 48;
const CAP_VENDOR: u8 = 0x09;
const COMMON_CFG: u8 = 1;

/// Whether the device may be confined: anything not virtio may; a virtio
/// device only when modern and `access_platform` reads Some(true).
pub(crate) fn admitted(vendor: u16, device: u16, access_platform: Option<bool>) -> bool {
    if vendor != VIRTIO_VENDOR || !(TRANSITIONAL.contains(&device) || MODERN.contains(&device)) {
        return true;
    }
    MODERN.contains(&device) && access_platform == Some(true)
}

/// Whether reading the feature word is needed at all.
pub(crate) fn is_modern_virtio(vendor: u16, device: u16) -> bool {
    vendor == VIRTIO_VENDOR && MODERN.contains(&device)
}

/// The common configuration window, (BAR index, offset in it), from the
/// capability list read through `read8` and `read32` (config offsets).
pub(crate) fn common_cfg(
    read8: impl Fn(u8) -> Option<u8>,
    read32: impl Fn(u8) -> Option<u32>,
) -> Option<(u8, u32)> {
    if read8(0x06)? & 0x10 == 0 {
        return None;
    }
    let mut at = read8(0x34)? & 0xFC;
    for _ in 0..MAX_CAPS {
        if at < 0x40 {
            return None;
        }
        if read8(at)? == CAP_VENDOR && read8(at.checked_add(3)?)? == COMMON_CFG {
            let bar = read8(at.checked_add(4)?)?;
            return (bar < 6).then_some((bar, read32(at.checked_add(8)?)?));
        }
        at = read8(at.checked_add(1)?)? & 0xFC;
    }
    None
}
