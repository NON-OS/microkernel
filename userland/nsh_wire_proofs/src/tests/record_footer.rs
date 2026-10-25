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

//! The record's footer must agree with its entries, and a record cut or
//! padded is refused.

use super::record::parse;
use crate::build::record::{EntryB, RecordB};
use nsh_wire::{Record, WireError};

#[test]
fn the_footer_must_agree_with_the_entries() {
    let d = RecordB::default;
    assert_eq!(parse(&RecordB { count: Some(1), ..d() }), Err(WireError::Value));
    assert_eq!(parse(&RecordB { count: Some(3), ..d() }), Err(WireError::Value));
    for end_reason in [0, 7, 255] {
        assert_eq!(parse(&RecordB { end_reason, ..d() }), Err(WireError::Value), "{end_reason}");
    }
    (1..=6).for_each(|end_reason| assert_eq!(parse(&RecordB { end_reason, ..d() }), Ok(())));
}

#[test]
fn a_cut_or_padded_record_is_refused() {
    let bytes = RecordB::default().bytes();
    assert_eq!(Record::parse(&bytes[..bytes.len() - 1]), Err(WireError::Short));
    /*
     * One byte more makes the footer read as an entry: its count and end
     * time as seq and time, its end reason as a command, and the first two
     * chain bytes (0xC4C4) as a body length over the 4096 cap.
     */
    let padded = [bytes, vec![0]].concat();
    assert_eq!(Record::parse(&padded), Err(WireError::Oversize));
    let extra = EntryB { seq: 2, at: 1_250, kind: 7, body: vec![0; 4] };
    let mut b = RecordB { count: Some(2), ..RecordB::default() };
    b.entries.push(extra);
    assert_eq!(parse(&b), Err(WireError::Value), "an entry the footer does not count");
}
