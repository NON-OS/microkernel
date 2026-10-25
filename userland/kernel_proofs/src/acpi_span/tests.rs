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

use super::entry_count::MAX_TABLE_BYTES;
use super::span::table_span;

const HEADER: usize = 36;
const MAPPED: u64 = 0x40_0000_0000;
const DMAR_AT: u64 = 0x7FEE_0000;

#[test]
fn a_table_within_the_map_is_summed_whole() {
    assert_eq!(table_span(DMAR_AT, 0x88, HEADER, MAPPED), Some(0x88));
    assert_eq!(table_span(DMAR_AT, HEADER as u32, HEADER, MAPPED), Some(HEADER));
    let largest = MAX_TABLE_BYTES as u32;
    assert_eq!(table_span(DMAR_AT, largest, HEADER, MAPPED), Some(largest as usize));
}

#[test]
fn a_length_under_the_header_or_past_the_bound_is_refused() {
    assert_eq!(table_span(DMAR_AT, HEADER as u32 - 1, HEADER, MAPPED), None);
    assert_eq!(table_span(DMAR_AT, 0, HEADER, MAPPED), None);
    assert_eq!(table_span(DMAR_AT, MAX_TABLE_BYTES as u32 + 1, HEADER, MAPPED), None);
    assert_eq!(table_span(DMAR_AT, u32::MAX, HEADER, MAPPED), None, "4 GiB of sum");
}

#[test]
fn a_table_running_past_the_map_is_refused() {
    let edge = MAPPED - 0x100;
    assert_eq!(table_span(edge, 0x100, HEADER, MAPPED), Some(0x100), "ends at the edge");
    assert_eq!(table_span(edge, 0x101, HEADER, MAPPED), None);
    assert_eq!(table_span(MAPPED, 0x40, HEADER, MAPPED), None, "starts past it");
    assert_eq!(table_span(u64::MAX - 8, 0x40, HEADER, u64::MAX), None, "no wrap");
}
