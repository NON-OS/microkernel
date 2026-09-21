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

//! Every way the chain check has to refuse a peer.

extern crate alloc;

use alloc::vec::Vec;

use crate::link::{bind, BindError};
use crate::vectors::{CAPTURED_AT, CERTS, IDENTITY, LEAF};

#[test]
fn a_different_identity_is_refused() {
    let mut other = IDENTITY;
    other[0] ^= 0x01;
    assert_eq!(bind(CERTS, LEAF, &other, CAPTURED_AT), Err(BindError::WrongIdentity));
}

#[test]
fn a_chain_from_another_session_is_refused() {
    let mut leaf: Vec<u8> = LEAF.to_vec();
    let last = leaf.len() - 1;
    leaf[last] ^= 0x01;
    assert_eq!(bind(CERTS, &leaf, &IDENTITY, CAPTURED_AT), Err(BindError::WrongSession));
}
