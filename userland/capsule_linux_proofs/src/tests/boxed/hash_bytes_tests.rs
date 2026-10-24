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

//! The strict box's hash, rebuilt from its documented bytes, and its domain.

use super::hash_tests::STRICT;
use crate::boxed::hash::{box_hash, DOMAIN};
use crate::boxed::preset::parse;
use crate::boxed::report::hex;

#[test]
fn the_hash_is_its_own_domain() {
    let p = parse(b"strict").unwrap();
    assert_eq!(DOMAIN, b"NONOS.linux.box.v1");
    assert_ne!(box_hash(&p), *blake3::hash(b"strict").as_bytes());
    let policy_domain = nonos_linux_policy::policy_hash(&[0; 256]);
    assert_ne!(box_hash(&p), policy_domain);
}

#[test]
fn the_strict_hash_is_its_documented_bytes() {
    // Domain, preset 1, TTY|EXEC big-endian, one folder "/tmp", no list.
    let mut bytes = b"NONOS.linux.box.v1".to_vec();
    bytes.push(1);
    bytes.extend_from_slice(&(0x140u64).to_be_bytes());
    bytes.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 4]);
    bytes.extend_from_slice(b"/tmp");
    bytes.extend_from_slice(&[0; 32]);
    assert_eq!(hex(blake3::hash(&bytes).as_bytes()), STRICT);
}
