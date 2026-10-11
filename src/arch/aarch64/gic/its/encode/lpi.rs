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

//! The redistributor's LPI tables: `GICR_PROPBASER`, `GICR_PENDBASER`, and
//! the configuration byte each LPI has in the first (IHI 0069, 5.1.1). Their
//! cacheability sits at 9:7, not where the ITS keeps it.

/// The priority every SPI gets, so an LPI neither starves nor preempts one.
const PRIORITY: u8 = 0xa0;
/// Bit 1 is RES1; Linux sets it as the group 1 bit some parts read it as.
const GROUP1: u8 = 1 << 1;
const ENABLE: u8 = 1;

pub const fn config(enabled: bool) -> u8 {
    PRIORITY | GROUP1 | if enabled { ENABLE } else { 0 }
}

const INNER_RAWAWB: u64 = 7 << 7;
const INNER_NC: u64 = 1 << 7;
const SH_INNER: u64 = 1 << 10;
/// The pending table is zero, so the redistributor need not read it.
const PTZ: u64 = 1 << 62;

const fn attrs(coherent: bool) -> u64 {
    if coherent {
        INNER_RAWAWB | SH_INNER
    } else {
        INNER_NC
    }
}

/// The configuration table at `phys` for INTIDs below `1 << id_bits`.
pub const fn propbaser(phys: u64, id_bits: u32, coherent: bool) -> u64 {
    (phys & 0x000f_ffff_ffff_f000) | attrs(coherent) | (id_bits.saturating_sub(1) & 0x1f) as u64
}

/// The zeroed pending table at `phys`, which must be 64 KiB aligned.
pub const fn pendbaser(phys: u64, coherent: bool) -> u64 {
    PTZ | (phys & 0x000f_ffff_ffff_0000) | attrs(coherent)
}

/// Configuration table bytes: one per LPI, the first for INTID 8192.
pub const fn prop_bytes(id_bits: u32) -> u64 {
    (1u64 << id_bits) - 8192
}

/// Pending table bytes: one bit per INTID, SPIs and below included.
pub const fn pend_bytes(id_bits: u32) -> u64 {
    (1u64 << id_bits) / 8
}
