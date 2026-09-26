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

//! The file span a program header names, on the offsets an ELF chooses.

use super::phdr::Phdr;

fn ph(offset: u64, filesz: u64) -> Phdr {
    Phdr { kind: 1, flags: 0, offset, vaddr: 0, filesz, memsz: filesz }
}

#[test]
fn a_header_names_exactly_its_bytes() {
    assert_eq!(ph(0x40, 0x10).file_range(), Some(0x40..0x50));
}

#[test]
fn an_empty_segment_is_an_empty_span() {
    assert_eq!(ph(0x1000, 0).file_range(), Some(0x1000..0x1000));
}

#[test]
fn a_span_ending_at_the_top_of_memory_is_kept() {
    let top = usize::MAX as u64;
    assert_eq!(ph(top - 4, 4).file_range(), Some(usize::MAX - 4..usize::MAX));
}

#[test]
fn a_span_that_wraps_is_refused() {
    let top = usize::MAX as u64;
    assert_eq!(ph(top - 3, 4).file_range(), None);
    assert_eq!(ph(u64::MAX, u64::MAX).file_range(), None);
}
