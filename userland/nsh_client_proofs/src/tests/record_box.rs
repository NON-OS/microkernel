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

//! The provisional BOX entry: read, flagged, and held to its body's
//! rules.

use super::record_good::check;
use crate::kit::record::{entry, Rec, START};
use nsh_client::{RecordRefusal, Verdict};
use nsh_wire::WireError;

fn complete(entries: u32, end_ms: u64, boxed: bool) -> Result<Verdict, RecordRefusal> {
    Ok(Verdict::Complete { entries, end_ms, end_reason: 1, boxed })
}

pub(super) fn with_box(body: &[u8]) -> Rec {
    let mut rec = Rec::good(3);
    rec.entries[3] = entry(3, START + 30, 9, body);
    rec.entries.push(entry(4, START + 40, 7, &0i32.to_le_bytes()));
    rec.end_ms = START + 45;
    rec
}

pub(super) fn box_body(preset: u8, started: u8) -> Vec<u8> {
    let mut b = vec![preset];
    b.extend_from_slice(&[0x58; 32]);
    b.push(started);
    b.extend_from_slice(&(-1i32).to_le_bytes());
    (0..9u64).for_each(|c| b.extend_from_slice(&c.to_le_bytes()));
    b
}

#[test]
fn a_box_entry_is_read_and_flagged_as_provisional() {
    assert_eq!(check(&with_box(&box_body(2, 1)).bytes(), &[]), complete(5, START + 45, true));
    assert_eq!(check(&with_box(&box_body(0, 0)).bytes(), &[]), complete(5, START + 45, true));
    let refused = Err(RecordRefusal::Parse(WireError::Value));
    assert_eq!(check(&with_box(&box_body(3, 1)).bytes(), &[]), refused);
    assert_eq!(check(&with_box(&box_body(0, 2)).bytes(), &[]), refused);
    let mut short = box_body(1, 1);
    short.pop();
    assert_eq!(check(&with_box(&short).bytes(), &[]), Err(RecordRefusal::Parse(WireError::Short)));
    let mut long = box_body(1, 1);
    long.push(0);
    let r = check(&with_box(&long).bytes(), &[]);
    assert_eq!(r, Err(RecordRefusal::Parse(WireError::Trailing)));
}
