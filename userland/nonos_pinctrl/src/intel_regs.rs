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

use crate::error::PadError;

// Linux pinctrl-intel.c: REVID at 0x000 carries the revision in its upper
// half, PADBAR at 0x00C the offset of the pad configuration array, and
// PADCFG0 bit 1 (GPIORXSTATE) the pad's input level after the glitch
// filter and before RX inversion.
pub const INTEL_REVID: u64 = 0x000;
pub const INTEL_PADBAR: u64 = 0x00C;
pub const INTEL_RXSTATE: u32 = 1 << 1;

/// Bytes between two pads' PADCFG0. Revision 0x092 and later add the
/// debounce dwords, four registers per pad instead of two (Linux
/// intel_pinctrl_probe_one sets PINCTRL_FEATURE_DEBOUNCE, intel_get_padcfg
/// uses nregs 4).
pub fn intel_pad_stride(revid: u32) -> u64 {
    if (revid >> 16) >= 0x092 {
        16
    } else {
        8
    }
}

/// Offset of pad `pad`'s PADCFG0 in a community window of `window` bytes
/// whose REVID and PADBAR read `revid` and `padbar`. A REVID of all ones is
/// a community that is not there (Linux returns -ENODEV for it).
pub fn intel_padcfg0(revid: u32, padbar: u32, pad: u16, window: u64) -> Result<u64, PadError> {
    if revid == u32::MAX || padbar == 0 || padbar == u32::MAX {
        return Err(PadError::Absent);
    }
    let off = u64::from(padbar) + u64::from(pad) * intel_pad_stride(revid);
    if off + 4 > window {
        return Err(PadError::OutsideWindow);
    }
    Ok(off)
}
