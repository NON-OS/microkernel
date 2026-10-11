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

//! The context descriptor a domain's streams share: its page tables, how
//! they are walked, and the ASID its TLB entries carry.

pub type Cd = [u64; 8];

/// 39-bit input addresses: three levels of 4 KiB tables from level 1.
pub const T0SZ: u64 = 25;
/// MAIR attribute 0, normal memory write-back, for every grant; attribute
/// 1, Device-nGnRE, for an MSI doorbell.
pub const MAIR: u64 = 0x04ff;

const TG0_4K: u64 = 0b00 << 6;
const IR0_WBWA: u64 = 0b01 << 8;
const OR0_WBWA: u64 = 0b01 << 10;
const SH0_INNER: u64 = 0b11 << 12;
const EPD1: u64 = 1 << 30;
const VALID: u64 = 1 << 31;
const AFFD: u64 = 1 << 35;
const AA64: u64 = 1 << 41;
/// Record faults and abort the faulting transaction rather than stalling.
const R: u64 = 1 << 45;
const A: u64 = 1 << 46;
const TTB_MASK: u64 = ((1 << 52) - 1) & !0xf;

/// A valid CD over the level 1 table at `ttb0`. TTB1 is disabled: device
/// addresses are never in the upper half.
pub const fn cd(ttb0: u64, asid: u16, ips: u8) -> Cd {
    let word0 = T0SZ
        | TG0_4K
        | IR0_WBWA
        | OR0_WBWA
        | SH0_INNER
        | EPD1
        | VALID
        | (ips as u64 & 0b111) << 32
        | AFFD
        | AA64
        | R
        | A
        | (asid as u64) << 48;
    [word0, ttb0 & TTB_MASK, 0, MAIR, 0, 0, 0, 0]
}
