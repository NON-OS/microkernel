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

use super::dma_lines::dma_sync_lines;

const VA: u64 = 0x7f00_0000_0000;

#[test]
fn an_aligned_buffer_flushes_exactly_its_lines() {
    assert_eq!(dma_sync_lines(VA, 4096), (VA, 64));
    assert_eq!(dma_sync_lines(VA, 64), (VA, 1));
    assert_eq!(dma_sync_lines(VA, 1), (VA, 1));
}

#[test]
fn a_range_that_straddles_lines_flushes_both_ends() {
    assert_eq!(dma_sync_lines(VA + 60, 8), (VA, 2));
    assert_eq!(dma_sync_lines(VA + 63, 1), (VA, 1));
    assert_eq!(dma_sync_lines(VA + 63, 2), (VA, 2));
    assert_eq!(dma_sync_lines(VA + 1, 4096), (VA, 65));
}

#[test]
fn an_empty_or_wrapping_range_flushes_nothing() {
    assert_eq!(dma_sync_lines(VA, 0).1, 0);
    assert_eq!(dma_sync_lines(u64::MAX - 10, 64).1, 0);
}
