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

//! The page from the diagnosis, arriving a tick at a time, and a kept
//! connection's second response.

use alloc::vec::Vec;

use super::app_reader_splits::{keys, records, trickle};
use crate::app_reader::AppReader;

/* The 91,628 byte page, arriving about 2.9 KB per tick. */
const PAGE: usize = 91_628;
const PER_TICK: usize = 2_900;

#[test]
fn a_page_arriving_by_the_tick_opens_each_record_once() {
    let body: Vec<u8> = (0..PAGE).map(|i| (i % 251) as u8).collect();
    let app = keys(0x1303);
    for (size, opens) in [(16_384, 6), (1_400, 66)] {
        let (wire, n) = records(&app, &body, size, 0);
        assert_eq!(n, opens);
        nonos_libc::reset();
        let (reader, opened) = trickle(&app, &wire, PER_TICK);
        assert_eq!(opened, opens, "records of {size} bytes, each opened once");
        assert_eq!(nonos_libc::counts().kernel_round_trips, 0, "opened in-process");
        assert_eq!(reader.plaintext(), &body[..]);
    }
}

#[test]
fn a_compacted_reader_reads_the_next_response() {
    let app = keys(0x1303);
    let first: Vec<u8> = (0..1000u32).map(|i| i as u8).collect();
    let second: Vec<u8> = (0..800u32).map(|i| (i * 3) as u8).collect();
    let (a, n) = records(&app, &first, 300, 0);
    let (b, _) = records(&app, &second, 300, n as u64);
    let mut reader = AppReader::new();
    let mut wire = a.clone();
    reader.feed(&app, &wire);
    assert_eq!(reader.plaintext(), &first[..]);
    reader.compact(&mut wire, first.len());
    assert!(wire.is_empty() && reader.plaintext().is_empty() && reader.cursor() == 0);
    wire.extend_from_slice(&b[..b.len() - 1]);
    reader.feed(&app, &wire);
    wire.push(*b.last().expect("byte"));
    reader.feed(&app, &wire);
    assert_eq!(reader.plaintext(), &second[..], "sequence numbers carried on");
    assert!(!reader.is_broken());
}
