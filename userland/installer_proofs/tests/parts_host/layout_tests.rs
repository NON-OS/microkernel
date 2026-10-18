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

//! Layout::parse directly: its refusals, and the offsets of what it accepts.

use crate::layout::{Layout, HEAD_LEN};
use crate::payloads::{head, store_payload};

#[test]
fn parse_refuses_a_short_head() {
    assert!(Layout::parse(&[]).is_none());
    assert!(Layout::parse(&[0; HEAD_LEN - 1]).is_none());
}

#[test]
fn parse_refuses_u32_max_lengths_alone_and_together() {
    for i in 0..5 {
        let mut lens = [0; 5];
        lens[i] = u32::MAX;
        assert!(Layout::parse(&head(0, lens)).is_none(), "length {i}");
    }
    assert!(Layout::parse(&head(0, [u32::MAX; 5])).is_none());
}

#[test]
fn parse_refuses_a_total_one_byte_off() {
    let mut p = store_payload(0, [3, 5, 7, 11, 13]);
    p.push(0);
    assert!(Layout::parse(&p).is_none());
    p.truncate(p.len() - 2);
    assert!(Layout::parse(&p).is_none());
}

#[test]
fn parse_takes_a_head_alone_with_every_offset_at_head_len() {
    let l = Layout::parse(&head(1, [0; 5])).expect("head alone parses");
    let lens = [l.elf_len, l.cert_len, l.manifest_len, l.trailer_len, l.args_len];
    assert_eq!(lens, [0; 5]);
    let ends = [l.after_elf, l.after_cert, l.after_manifest, l.after_trailer, l.total];
    assert_eq!(ends, [HEAD_LEN; 5]);
}

#[test]
fn parse_offsets_are_running_sums_of_the_lengths() {
    let l = Layout::parse(&store_payload(1, [3, 5, 7, 11, 13])).expect("parses");
    let lens = [l.elf_len, l.cert_len, l.manifest_len, l.trailer_len, l.args_len];
    assert_eq!(lens, [3, 5, 7, 11, 13]);
    assert_eq!(l.after_elf, HEAD_LEN + 3);
    assert_eq!(l.after_cert, HEAD_LEN + 3 + 5);
    assert_eq!(l.after_manifest, HEAD_LEN + 3 + 5 + 7);
    assert_eq!(l.after_trailer, HEAD_LEN + 3 + 5 + 7 + 11);
    assert_eq!(l.total, HEAD_LEN + 3 + 5 + 7 + 11 + 13);
}
