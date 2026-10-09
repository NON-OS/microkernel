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

use super::queue::{iqa_value, queue_free, queue_index, queue_next, queue_offset, QUEUE_ENTRIES};

/* 11.4.9.3: IQA holds the base in bits 63:12, DW in bit 11, QS in 2:0. */
#[test]
fn iqa_is_one_page_of_narrow_descriptors() {
    assert_eq!(iqa_value(0x1_2345_6000), 0x1_2345_6000);
    assert_eq!(iqa_value(0x1_2345_6FFF) & 0xFFF, 0, "DW and QS zero");
}

/* 11.4.9.1 and 11.4.9.2: head and tail hold the index in bits 18:4. */
#[test]
fn head_and_tail_are_sixteen_byte_offsets() {
    assert_eq!(queue_offset(0), 0);
    assert_eq!(queue_offset(255), 0xFF0);
    for i in 0..QUEUE_ENTRIES {
        assert_eq!(queue_index(queue_offset(i)), i);
    }
    assert_eq!(queue_index(0xF | 0x30), 3, "low bits are reserved");
}

#[test]
fn the_ring_wraps_and_never_fills_its_last_slot() {
    assert_eq!(queue_next(254), 255);
    assert_eq!(queue_next(255), 0);
    assert_eq!(queue_free(0, 0), QUEUE_ENTRIES - 1, "empty");
    assert_eq!(queue_free(5, 4), 0, "full: tail one behind head");
    assert_eq!(queue_free(0, 255), 0, "full across the wrap");
    assert_eq!(queue_free(4, 5), QUEUE_ENTRIES - 2);
}

/* A unit that drains each batch before the next one, as `submit` waits for,
never runs out of room however often the tail wraps; one that stops draining
is caught before software overwrites a slot it has not fetched. */
#[test]
fn submit_model_wraps_and_detects_a_stalled_unit() {
    let (mut head, mut tail) = (0u16, 0u16);
    for batch in 1..2000usize {
        let len = batch % 3 + 1;
        assert!(queue_free(head, tail) as usize > len);
        for _ in 0..=len {
            tail = queue_next(tail);
        }
        head = tail;
    }
    let mut stalled = 0;
    head = tail;
    while queue_free(head, tail) as usize >= 3 {
        tail = queue_next(queue_next(queue_next(tail)));
        stalled += 1;
    }
    assert_eq!(stalled, (QUEUE_ENTRIES as usize - 1) / 3);
}
