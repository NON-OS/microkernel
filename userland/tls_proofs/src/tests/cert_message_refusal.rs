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

//! Certificate messages that must not yield a certificate.

use super::cert_message_vectors::message;
use crate::cert_at::cert_at;
use crate::cert_count::cert_count;
use crate::example_ca::EXAMPLE_CA;
use crate::example_leaf::EXAMPLE_LEAF;

#[test]
fn trailing_bytes_are_refused() {
    let mut body = message(&[EXAMPLE_LEAF, EXAMPLE_CA]);
    body.push(0x00);
    assert_eq!(cert_count(&body), 0, "the list length must cover the body exactly");
    assert!(cert_at(&body, 0).is_none());
}
#[test]
fn an_overlong_list_length_is_refused() {
    let mut body = message(&[EXAMPLE_LEAF]);
    body[3] = body[3].wrapping_add(1);
    assert_eq!(cert_count(&body), 0);
    assert!(cert_at(&body, 0).is_none());
}
#[test]
fn no_prefix_of_a_message_yields_a_certificate() {
    let body = message(&[EXAMPLE_LEAF, EXAMPLE_CA]);
    for cut in (0..body.len()).step_by(11) {
        let part = &body[..cut];
        assert_eq!(cert_count(part), 0, "a truncated message holds no complete list");
        assert!(cert_at(part, 0).is_none(), "and yields no certificate at {cut}");
    }
}
#[test]
fn a_non_empty_request_context_is_skipped() {
    let inner = message(&[EXAMPLE_LEAF]);
    let mut body = vec![4u8, 0xAA, 0xBB, 0xCC, 0xDD];
    body.extend_from_slice(&inner[1..]);
    assert_eq!(cert_count(&body), 1);
    assert_eq!(cert_at(&body, 0).expect("leaf"), EXAMPLE_LEAF);
}
