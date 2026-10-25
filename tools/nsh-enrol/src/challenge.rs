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

//! The challenge an enrolment's document must answer: fresh, from a nonce
//! the operator draws, and naming the host keys being enrolled. A document
//! made for it shows the machine holding that AK was asked, now, to vouch
//! for these keys; an old document, or one asked for other keys, is not it.

use nsh_crypto::key_digest;

use crate::keys::PUBLIC_FILE_LEN;

const LABEL: &[u8] = b"nsh-enrol challenge\0";

pub fn challenge(nonce: &[u8; 32], host_keys: &[u8; PUBLIC_FILE_LEN]) -> [u8; 32] {
    let keys = key_digest(&host_keys[..32], &host_keys[32..]);
    nonos_hash::sha256(&[LABEL, &nonce[..], &keys[..]].concat())
}
