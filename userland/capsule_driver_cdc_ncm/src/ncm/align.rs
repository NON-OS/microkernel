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

//! Where an NDP and a datagram start in an NTB sent to the device: the
//! device's wNdpOutAlignment, wNdpOutDivisor and wNdpOutPayloadRemainder
//! (NCM 1.0, table 6-3), checked as Linux cdc_ncm_fix_modulus
//! checks them.

use nonos_usbnet::nic::ETH_HEADER;

use super::params::NtbParams;

/// USB_CDC_NCM_NDP_ALIGN_MIN_SIZE: what a value the device got wrong
/// falls back to.
const ALIGN_MIN: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OutAlign {
    pub ndp: usize,
    pub modulus: usize,
    /// Where a datagram starts within `modulus`: chosen so the payload
    /// after its Ethernet header lands on the device's remainder.
    pub remainder: usize,
}

pub fn out_align(p: &NtbParams, tx_max: usize) -> OutAlign {
    // A power of two of at least four, and less than the largest block.
    let sane = |v: u16| {
        let v = v as usize;
        v >= ALIGN_MIN && v.is_power_of_two() && v < tx_max
    };
    let ndp = if sane(p.out_alignment) { p.out_alignment as usize } else { ALIGN_MIN };
    let modulus = if sane(p.out_divisor) { p.out_divisor as usize } else { ALIGN_MIN };
    let given = p.out_remainder as usize;
    let given = if given < modulus { given } else { 0 };
    let remainder = given.wrapping_sub(ETH_HEADER) & (modulus - 1);
    OutAlign { ndp, modulus, remainder }
}

/// Linux cdc_ncm_align_tail: from `len` to the next multiple of `modulus`
/// plus `remainder`, never past `max`.
pub fn align_tail(len: usize, modulus: usize, remainder: usize, max: usize) -> usize {
    (len.next_multiple_of(modulus) + remainder).min(max).max(len)
}
