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

//! Every record shape the seal never writes is refused with a reason, and a
//! record far larger than any record is refused before it is parsed.

use super::record_tests::{rec, H};
use crate::release::record::{parse, MAX_BYTES};
use serde_json::json;

#[test]
fn every_other_shape_is_refused_with_a_reason() {
    let ok = json!({"usb": {"file": "a", "sha256": H}});
    let cases: Vec<Vec<u8>> = vec![
        b"not json".to_vec(),
        b"[]".to_vec(),
        serde_json::to_vec(
            &json!({"format": "nonos-release/2", "config": {"release": true}, "sealed": ok}),
        )
        .unwrap(),
        serde_json::to_vec(&json!({"config": {"release": true}, "sealed": ok})).unwrap(),
        rec(json!({"release": "yes"}), ok.clone()),
        rec(json!({"release": true, "features": "nonos-dev-attest"}), ok.clone()),
        rec(json!({"release": true, "features": [1]}), ok.clone()),
        rec(json!({"release": true, "rollback_index": -1}), ok.clone()),
        rec(json!({"release": true}), json!({})),
        rec(json!({"release": true}), json!({"u sb": {"file": "a", "sha256": H}})),
        rec(json!({"release": true}), json!({"usb": {"file": "a", "dir": "b", "sha256": H}})),
        rec(json!({"release": true}), json!({"usb": {"sha256": H}})),
    ];
    for c in cases {
        let e = parse(&c).expect_err(&String::from_utf8_lossy(&c));
        assert!(!e.is_empty());
    }
}

#[test]
fn a_record_far_larger_than_any_record_is_refused_unparsed() {
    let big = vec![b' '; MAX_BYTES as usize + 1];
    assert!(parse(&big).unwrap_err().contains("far more"));
}
