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

//! Memory attributes for what the ITS reads, and `GITS_CBASER`, its command
//! queue. `GITS_BASER<n>` and `GITS_CBASER` put cacheability at 61:59 and
//! shareability at 11:10, so one encoding serves both.

const VALID: u64 = 1 << 63;
const INNER_RAWAWB: u64 = 7 << 59;
const INNER_NC: u64 = 1 << 59;
const SH_INNER: u64 = 1 << 10;
const SH_MASK: u64 = 3 << 10;

/// Write-back and inner shareable, unless the ITS said by reading back
/// non-shareable that it does not snoop: then non-cacheable, and the
/// kernel cleans what it writes out to memory itself.
pub const fn attrs(coherent: bool) -> u64 {
    if coherent {
        INNER_RAWAWB | SH_INNER
    } else {
        INNER_NC
    }
}

/// Whether a written register read back shareable.
pub const fn coherent(readback: u64) -> bool {
    readback & SH_MASK != 0
}

/// A valid command queue of `pages` 4 KiB pages at `phys`.
pub const fn cbaser(phys: u64, pages: u64, coherent: bool) -> u64 {
    VALID | attrs(coherent) | (phys & 0x000f_ffff_ffff_f000) | (pages - 1)
}
