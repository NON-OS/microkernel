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

//! One frame as an NTB16 (NCM 1.0, 3.2 and 3.3): the NTH16, an NDP16 with
//! one entry and the zero entry that ends it, and the datagram, placed as
//! Linux cdc_ncm_fill_tx_frame places the first datagram of a block.

use super::align::align_tail;
use super::pad::padded;
use super::shape::TxShape;

/// "NCMH" and "NCM0" (USB_CDC_NCM_NTH16_SIGN, USB_CDC_NCM_NDP16_NOCRC_SIGN).
pub const NTH16_SIGN: u32 = 0x484D_434E;
pub const NDP16_NOCRC_SIGN: u32 = 0x304D_434E;
pub const NTH16_LEN: usize = 12;
/// An NDP16 with one entry and its terminator, USB_CDC_NCM_NDP16_LENGTH_MIN.
pub const NDP16_ONE: usize = 16;

/// Lay `frame` out in `out` as block `seq`: its length, or `None` when it
/// does not fit in `s.max` bytes (Linux drops it: "won't fit, MTU
/// problem?").
pub fn build(out: &mut [u8], frame: &[u8], seq: u16, s: &TxShape) -> Option<usize> {
    let (max, a) = (s.max, s.align);
    if out.len() < max {
        return None;
    }
    let ndp = align_tail(NTH16_LEN, a.ndp, 0, max);
    // cdc_ncm_ndp16: room for the NDP, the datagram and its alignment.
    let reserve = frame.len() + a.modulus + a.remainder;
    if max.checked_sub(ndp)?.checked_sub(reserve)? < NDP16_ONE {
        return None;
    }
    let at = align_tail(ndp + NDP16_ONE, a.modulus, a.remainder, max);
    let end = at + frame.len();
    if end > max {
        return None;
    }
    let len = padded(end, s);
    out[..len].fill(0);
    put(out, 0, &NTH16_SIGN.to_le_bytes());
    let nth = [NTH16_LEN as u16, seq, len as u16, ndp as u16];
    nth.iter().enumerate().for_each(|(i, v)| put(out, 4 + 2 * i, &v.to_le_bytes()));
    put(out, ndp, &NDP16_NOCRC_SIGN.to_le_bytes());
    // wLength, wNextNdpIndex 0, the one entry; the zero entry is the fill.
    let ndp16 = [NDP16_ONE as u16, 0, at as u16, frame.len() as u16];
    ndp16.iter().enumerate().for_each(|(i, v)| put(out, ndp + 4 + 2 * i, &v.to_le_bytes()));
    out[at..end].copy_from_slice(frame);
    Some(len)
}

fn put(out: &mut [u8], at: usize, bytes: &[u8]) {
    out[at..at + bytes.len()].copy_from_slice(bytes);
}
