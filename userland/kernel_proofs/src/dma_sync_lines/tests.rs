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

#[test]
fn any_power_of_two_line_steps_over_every_byte_once() {
    use super::dma_lines::dma_sync_lines_of;
    for line in [16u64, 32, 64, 128, 256, 2048] {
        for off in 0..line + 3 {
            for len in [1u64, line - 1, line, line + 1, 3 * line + 5] {
                let (first, n) = dma_sync_lines_of(VA + off, len, line);
                assert_eq!(first % line, 0);
                assert!(first <= VA + off && VA + off - first < line);
                assert!(first + n * line >= VA + off + len, "covers the last byte");
                assert!(first + (n - 1) * line < VA + off + len, "no line past it");
            }
        }
    }
    assert_eq!(dma_sync_lines_of(VA, 0, 16).1, 0);
}

#[test]
fn ctr_dminline_is_words_as_a_power_of_two() {
    use super::dma_lines::ctr_dminline_bytes;
    assert_eq!(ctr_dminline_bytes(4 << 16), 64, "DminLine 4: sixteen words");
    assert_eq!(ctr_dminline_bytes(0x8444_c004), 64, "other CTR_EL0 fields ignored");
    assert_eq!(ctr_dminline_bytes(0), 4);
    assert_eq!(ctr_dminline_bytes(0xf << 16), 4 << 15);
}
