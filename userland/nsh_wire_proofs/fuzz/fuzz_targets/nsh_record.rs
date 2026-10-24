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

//! A record: one that parses has entries numbered from 0 without a gap,
//! times that never go back, and a footer that agrees with them.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_wire::record::BOX_BODY;
use nsh_wire::{EntryBody, Record};

fuzz_target!(|data: &[u8]| {
    if let Ok(r) = Record::parse(data) {
        let mut last = r.header.start_ms;
        let mut n = 0u32;
        for e in r.entries() {
            assert_eq!(e.seq, n);
            assert!(e.at_ms >= last);
            if let EntryBody::Box(b) = e.body {
                assert_eq!(e.bytes.len(), 4 + 8 + 1 + 2 + BOX_BODY);
                assert!(b.counts.iter().all(|c| *c <= b.refused()));
            }
            (last, n) = (e.at_ms, n + 1);
        }
        assert_eq!(n, r.footer.count);
        assert!(r.footer.end_ms >= last);
        assert!(data.starts_with(r.header.bytes) && data.starts_with(r.signed));
    }
});
