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

//! Memory figures the inspector and the Memory screen share.

use super::format::{mem_human, u32_decimal};
use super::state::Row;

/* "0.7%": `kb` as a share of `total_kb` to the tenth, clamped; zero of
 * nothing is zero. A whole percent printed 0.0% for every process under a
 * hundredth of RAM. */
pub fn share_1dp(kb: u64, total_kb: u64, out: &mut [u8]) -> usize {
    let tenths = if total_kb == 0 { 0 } else { (kb.saturating_mul(1000) / total_kb).min(1000) };
    let mut n = u32_decimal((tenths / 10) as u32, out);
    if n + 3 <= out.len() {
        out[n] = b'.';
        out[n + 1] = b'0' + (tenths % 10) as u8;
        out[n + 2] = b'%';
        n += 3;
    }
    n
}

/* A Linux guest's mappings are kept by its supervisor and the kernel lists
 * none, so the cell names who holds them instead of printing 0 KB. */
pub fn mapped<'a>(row: &Row, out: &'a mut [u8]) -> &'a [u8] {
    if row.name().starts_with(b"foreign:") {
        return b"supervisor";
    }
    let n = mem_human(row.mapped_kb, out);
    &out[..n]
}
