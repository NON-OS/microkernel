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

//! A policy's vouched measurement is its own domain's: no plain blob, and no
//! other domain's, gives it.

use crate::args::VOUCH_DOMAIN as ARGS_DOMAIN;
use crate::vouch::{in_a_domain, vouched_hash, RESERVED, VOUCH_DOMAIN};
use crate::{policy_hash, MAGIC};

#[test]
fn the_vouched_hash_is_the_domain_then_the_policy() {
    let policy = [7u8; 256];
    let whole = [VOUCH_DOMAIN, &policy[..]].concat();
    assert_eq!(vouched_hash(&policy), *blake3::hash(&whole).as_bytes());
    assert_ne!(vouched_hash(&policy), *blake3::hash(&policy).as_bytes());
    assert_eq!(vouched_hash(&policy), policy_hash(&policy), "one hash a policy is known by");
}

#[test]
fn both_domains_are_reserved_and_no_program_or_policy_is() {
    assert!(VOUCH_DOMAIN.starts_with(RESERVED) && ARGS_DOMAIN.starts_with(RESERVED));
    assert!(in_a_domain(VOUCH_DOMAIN) && in_a_domain(ARGS_DOMAIN));
    for plain in [&b"\x7fELF\x02\x01"[..], b"#!/bin/sh", &MAGIC[..], b"", b"NONOS.linu"] {
        assert!(!in_a_domain(plain), "{plain:?} is minted plain");
    }
}

#[test]
fn the_policy_and_args_domains_never_give_one_another_s_bytes() {
    assert!(!VOUCH_DOMAIN.starts_with(ARGS_DOMAIN) && !ARGS_DOMAIN.starts_with(VOUCH_DOMAIN));
}
