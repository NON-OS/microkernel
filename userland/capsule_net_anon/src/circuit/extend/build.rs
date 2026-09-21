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

//! Building the EXTEND2 body, specifiers and handshake together.

extern crate alloc;

use alloc::vec::Vec;

use crate::cell::HANDSHAKE_NTOR;

use super::types::NextHop;

// Link specifier types, from the fork's src/trunnel/link_specifier.trunnel.
const SPEC_IPV4: u8 = 0;
const SPEC_RSA_ID: u8 = 2;
const SPEC_ED25519_ID: u8 = 3;

/// An EXTEND2 body: a count of link specifiers, the specifiers, then the
/// handshake.
pub fn extend2_body(next: &NextHop, onionskin: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(64 + onionskin.len());
    out.push(3);

    out.push(SPEC_IPV4);
    out.push(6);
    out.extend_from_slice(&next.address);
    out.extend_from_slice(&next.port.to_be_bytes());

    out.push(SPEC_RSA_ID);
    out.push(20);
    out.extend_from_slice(&next.rsa_identity);

    out.push(SPEC_ED25519_ID);
    out.push(32);
    out.extend_from_slice(&next.ed25519_identity);

    out.extend_from_slice(&HANDSHAKE_NTOR.to_be_bytes());
    out.extend_from_slice(&(onionskin.len() as u16).to_be_bytes());
    out.extend_from_slice(onionskin);
    out
}
