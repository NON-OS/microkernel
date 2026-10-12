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

// An Intel PCH GPIO community (the PCH datasheets, volume 2, GPIO
// registers): the revision register at 0x000 with the revision in its
// upper half, the pad base register at 0x00C holding the offset of the pad
// configuration array, and PAD_CFG_DW0 bit 1 the pad's RX state. Recalled,
// not read: unverified.
pub const INTEL_REVID: u64 = 0x000;
pub const INTEL_PADBAR: u64 = 0x00C;
pub const INTEL_RXSTATE: u32 = 1 << 1;

/// Bytes between two pads' PAD_CFG_DW0: 8 on the older communities, 16 on
/// those whose pads carry the further configuration dwords. Which revision
/// value marks the change (0x092 here) is recalled without a published
/// source to cite: unverified.
pub fn intel_pad_stride(revid: u32) -> u64 {
    if (revid >> 16) >= 0x092 {
        16
    } else {
        8
    }
}

/// Offset of pad `pad`'s PAD_CFG_DW0 in a community window of `window`
/// bytes whose revision and pad base read `revid` and `padbar`. A revision
/// of all ones, or a pad base of zero or all ones, is a community that does
/// not decode.
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
