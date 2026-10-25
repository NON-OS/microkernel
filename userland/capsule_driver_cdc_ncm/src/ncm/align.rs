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

//! Where an NDP and a datagram start in an NTB sent to the device, from
//! its wNdpOutAlignment, wNdpOutDivisor and wNdpOutPayloadRemainder (NCM
//! 1.0, table 6-3, as recalled, unverified): NDPs on the alignment, and
//! each datagram's payload, the bytes after its Ethernet header, at an
//! offset whose remainder by the divisor is the one given.

use nonos_usbnet::nic::ETH_HEADER;

use super::params::NtbParams;

/// NDPs start on four-byte boundaries whatever else is asked (NCM 1.0,
/// 3.3); a value the device got wrong falls back to it.
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

/// The first offset at or after `at` whose remainder by `modulus`, a power
/// of two, is `remainder`.
pub fn next(at: usize, modulus: usize, remainder: usize) -> usize {
    at + (remainder.wrapping_sub(at) & (modulus - 1))
}
