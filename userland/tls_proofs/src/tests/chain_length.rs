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

//! How much signature work a server's Certificate message can ask for, and
//! what reaches the pool's frame.

use super::cert_message_vectors::message;
use crate::chain_walk::verify_chain;
use crate::example_leaf::EXAMPLE_LEAF;

/// Inside the example leaf's validity window, as YYYYMMDDhhmmss.
const INSIDE: u64 = 20260920000000;

fn pool_calls() -> u32 {
    let c = nonos_libc::counts();
    c.rsa + c.p256 + c.p384
}

/*
 * Every link of a served chain costs a signature check in the pool, and the
 * count field allows 255 entries. Real chains are a leaf and one to three
 * issuers; ten covers cross-signed paths with room to spare.
 */
#[test]
fn a_chain_longer_than_ten_is_refused_before_any_signature() {
    let body = message(&[EXAMPLE_LEAF; 11]);
    nonos_libc::reset();
    assert!(!verify_chain(&body, b"example.com", INSIDE));
    assert_eq!(pool_calls(), 0, "refused by its length, not by a signature");
}

#[test]
fn a_chain_of_ten_is_still_walked() {
    let body = message(&[EXAMPLE_LEAF; 10]);
    nonos_libc::reset();
    assert!(!verify_chain(&body, b"example.com", INSIDE), "the leaf did not sign itself");
    assert!(pool_calls() >= 1, "a chain inside the bound reaches the signature check");
}

/*
 * The pool frame carries the key and signature lengths in two bytes each. A
 * longer key or signature would be cut to its low sixteen bits and the bytes
 * after it read as the next field.
 */
#[test]
fn a_key_or_signature_too_long_for_the_frame_never_reaches_the_pool() {
    let digest = [0u8; 32];
    let big = vec![0x30u8; 70_000];
    nonos_libc::reset();
    assert!(!crate::verify_rsa::verify_rsa(0, 0, &big, &[1, 2, 3], &digest));
    assert!(!crate::verify_rsa::verify_rsa(0, 0, &[0x30, 0], &big, &digest));
    assert_eq!(nonos_libc::counts().rsa, 0);
}
