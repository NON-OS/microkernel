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

//! Where the MSI capability keeps each register, and the control words the
//! broker writes. Pure, so the layout is held against the spec on the host
//! (kernel_proofs msi_layout).
//!
//! PCI Local Bus 3.0, 6.8.1: the message control word sits at +2, the low
//! address at +4. A 64-bit capable function puts the high address at +8 and
//! the data at +0xC, a 32-bit one the data at +8. With per-vector masking the
//! mask bits follow the data at +0x10 (64-bit) or +0xC (32-bit).

/// Message Control bit 0, MSI Enable.
pub(crate) const CTRL_ENABLE: u16 = 1 << 0;
/// Message Control bits 6:4, Multiple Message Enable.
pub(crate) const CTRL_MME: u16 = 0x7 << 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MsiLayout {
    pub(crate) control: u16,
    pub(crate) address_lo: u16,
    pub(crate) address_hi: Option<u16>,
    pub(crate) data: u16,
    pub(crate) mask: Option<u16>,
}

pub(crate) const fn layout(cap: u16, is_64bit: bool, per_vector_mask: bool) -> MsiLayout {
    let data = if is_64bit { cap + 0xC } else { cap + 0x8 };
    MsiLayout {
        control: cap + 2,
        address_lo: cap + 4,
        address_hi: if is_64bit { Some(cap + 8) } else { None },
        data,
        mask: if per_vector_mask { Some(data + 4) } else { None },
    }
}

/// The control word that enables MSI with one message: MME = 0 means one
/// vector (6.8.1.3), the only size the broker pool hands out.
pub(crate) const fn enable_one(ctrl: u16) -> u16 {
    (ctrl & !CTRL_MME) | CTRL_ENABLE
}

pub(crate) const fn disable(ctrl: u16) -> u16 {
    ctrl & !CTRL_ENABLE
}

/// Mask bits that leave only vector 0 open, over the vectors the function
/// can raise (Multiple Message Capable, log2 of the count). Linux msi_mask.
pub(crate) const fn open_first(multi_message_capable: u8) -> u32 {
    let count = 1u32 << (multi_message_capable & 0x7);
    let all = if count >= 32 { u32::MAX } else { (1u32 << count) - 1 };
    all & !1
}
