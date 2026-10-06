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

use super::flags::{DMA_MAP_COHERENT, DMA_MAP_DMA32, DMA_MAP_HIGH, DMA_MAP_WC};
use super::value::{entry, index, with_wc, MT_UC, MT_WB, MT_WC, PAT_RESET};

/* map_user_dma_coherent: PCD+PWT uncached, PWT alone write-combining. */
const UNCACHED: (bool, bool, bool) = (false, true, true);
const WRITE_COMBINING: (bool, bool, bool) = (false, false, true);
const WRITE_BACK: (bool, bool, bool) = (false, false, false);

fn type_of(pat: u64, bits: (bool, bool, bool)) -> u8 {
    entry(pat, index(bits.0, bits.1, bits.2))
}

#[test]
fn a_coherent_grant_is_strong_uncached_with_or_without_the_wc_entry() {
    assert_eq!(type_of(with_wc(PAT_RESET), UNCACHED), MT_UC);
    assert_eq!(type_of(PAT_RESET, UNCACHED), MT_UC);
}

#[test]
fn a_write_combining_grant_is_wc_once_the_pat_has_the_entry() {
    assert_eq!(type_of(with_wc(PAT_RESET), WRITE_COMBINING), MT_WC);
}

#[test]
fn a_plain_grant_stays_write_back() {
    assert_eq!(type_of(with_wc(PAT_RESET), WRITE_BACK), MT_WB);
    assert_eq!(type_of(PAT_RESET, WRITE_BACK), MT_WB);
}

#[test]
fn every_map_flag_is_its_own_bit_as_the_libc_names_it() {
    let all = [DMA_MAP_HIGH, DMA_MAP_DMA32, DMA_MAP_COHERENT, DMA_MAP_WC];
    assert_eq!(all, [1, 2, 4, 8]);
    assert_eq!(all.iter().fold(0, |acc, f| acc | f).count_ones(), 4);
}
