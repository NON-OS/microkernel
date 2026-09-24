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

//! The consensus header, its validity window and its weights.

use crate::directory::consensus::span::signed_range;
use crate::directory::consensus::{is_stale, parse};
use crate::vectors::CONSENSUS;

#[test]
fn live_consensus_parses() {
    let doc = parse(CONSENSUS).expect("the live consensus parses");
    // valid-after 2026-09-18 19:00:00, fresh-until 20:00:00, valid-until 22:00.
    assert_eq!(doc.valid_after, 1_789_758_000);
    assert_eq!(doc.fresh_until, doc.valid_after + 3_600);
    assert_eq!(doc.valid_until, doc.valid_after + 3 * 3_600);
    assert_eq!(doc.entries.len(), 6, "every relay in the vector was kept");
    assert_eq!(doc.signatures.len(), 7, "all seven authorities signed");
}

#[test]
fn validity_window_is_enforced_at_both_ends() {
    let doc = parse(CONSENSUS).expect("parses");
    assert!(!doc.valid_at(doc.valid_after - 1));
    assert!(doc.valid_at(doc.valid_after));
    assert!(doc.valid_at(doc.valid_until - 1));
    assert!(!doc.valid_at(doc.valid_until), "expiry must be exclusive");
    assert!(!is_stale(doc.fresh_until, doc.fresh_until - 1));
    assert!(is_stale(doc.fresh_until, doc.fresh_until));
}

#[test]
fn the_published_weights_are_read() {
    let doc = parse(CONSENSUS).expect("parses");
    // bandwidth-weights Wgg=10000 Wgd=1893 Wmd=4411 Wed=3696 Wee=10000 ...
    assert_eq!(doc.weights.wgg, 10_000);
    assert_eq!(doc.weights.wgd, 1_893);
    assert_eq!(doc.weights.wmd, 4_411);
    assert_eq!(doc.weights.wed, 3_696);
    assert_eq!(doc.weights.wee, 10_000);
}

#[test]
fn the_signed_range_ends_inside_the_first_signature_line() {
    let (start, end) = signed_range(CONSENSUS).expect("range found");
    assert_eq!(start, 0, "the document opens with network-status-version");
    assert_eq!(&CONSENSUS[end - 20..end], b"directory-signature ");
    assert!(end < CONSENSUS.len());
}
