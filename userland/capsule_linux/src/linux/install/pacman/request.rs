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

//! An OpenPGP RSA check, put as the crypto service takes it.

use alloc::vec::Vec;

use super::spki::rsa_spki;

/// What the crypto service is asked: its hash number, the key, and the
/// signature at the modulus's width (an MPI drops leading zeros).
pub struct Request {
    pub hashid: u8,
    pub spki: Vec<u8>,
    pub sig: Vec<u8>,
}

/// OpenPGP's SHA-256 and SHA-512 are the service's 0 and 2; nothing else.
pub fn request(n: &[u8], e: &[u8], sig: &[u8], hash: u8) -> Option<Request> {
    let hashid = match hash {
        8 => 0,
        10 => 2,
        _ => return None,
    };
    let mut full = alloc::vec![0u8; n.len().checked_sub(sig.len())?];
    full.extend_from_slice(sig);
    Some(Request { hashid, spki: rsa_spki(n, e)?, sig: full })
}
