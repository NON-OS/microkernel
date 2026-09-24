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

//! Certificates come back out of a message as they went in.

use super::cert_message_vectors::message;
use crate::cert_at::cert_at;
use crate::cert_count::cert_count;
use crate::example_ca::EXAMPLE_CA;
use crate::example_leaf::EXAMPLE_LEAF;

#[test]
fn both_certificates_come_back_exactly_as_they_went_in() {
    let body = message(&[EXAMPLE_LEAF, EXAMPLE_CA]);
    assert_eq!(cert_count(&body), 2);
    assert_eq!(cert_at(&body, 0).expect("leaf"), EXAMPLE_LEAF);
    assert_eq!(cert_at(&body, 1).expect("issuer"), EXAMPLE_CA);
}
#[test]
fn the_count_and_the_accessor_agree() {
    for certs in [
        vec![EXAMPLE_LEAF],
        vec![EXAMPLE_LEAF, EXAMPLE_CA],
        vec![EXAMPLE_LEAF, EXAMPLE_CA, EXAMPLE_CA],
    ] {
        let body = message(&certs);
        let n = cert_count(&body);
        assert_eq!(n as usize, certs.len());
        for i in 0..n {
            assert!(cert_at(&body, i).is_some(), "entry {i} of {n} must be readable");
        }
        assert!(cert_at(&body, n).is_none(), "one past the count is not there");
    }
}
#[test]
fn an_empty_list_holds_nothing() {
    let body = message(&[]);
    assert_eq!(cert_count(&body), 0);
    assert!(cert_at(&body, 0).is_none());
}
