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


//! The relay messages a client sends and reads to reach an onion service
//! (the fork's hs_cell.c and src/trunnel/hs/*.trunnel).

extern crate alloc;

use alloc::vec::Vec;

use nonos_aes::Ctr256Be;

use crate::crypto::keccak::mac_sha3_256;

use super::hs_ntor::IntroKeys;
use super::pow::EXTENSION_BYTES;

/* Relay commands, or.h. */
pub const RELAY_BEGIN_DIR: u8 = 13;
pub const RELAY_ESTABLISH_RENDEZVOUS: u8 = 33;
pub const RELAY_INTRODUCE1: u8 = 34;
pub const RELAY_RENDEZVOUS2: u8 = 37;
pub const RELAY_RENDEZVOUS_ESTABLISHED: u8 = 39;
pub const RELAY_INTRODUCE_ACK: u8 = 40;

pub const COOKIE_BYTES: usize = 20;
/// HS_CELL_INTRODUCE1_MIN_SIZE: the cleartext and the plaintext of the
/// encrypted part are padded to at least this, so INTRODUCE1 cells do not
/// differ in length by what they carry.
const INTRODUCE1_MIN: usize = 246;
const AUTH_KEY_ED25519: u8 = 2;
const ONION_KEY_NTOR: u8 = 1;

/// INTRODUCE1 to the introduction point whose service auth key is `auth`,
/// asking the service to meet us at the rendezvous point with `cookie`,
/// whose ntor key is `rp_onion_key` and whose link specifiers (count
/// first) are `rp_specs`. `client` is our X; `keys` from `intro_keys`.
/// `pow` is a solved puzzle's extension field, carried in the encrypted
/// part where only the service reads it. `None` when the cell would not
/// fit in one relay message.
pub fn introduce1(
    auth: &[u8; 32],
    client: &[u8; 32],
    keys: &IntroKeys,
    cookie: &[u8; COOKIE_BYTES],
    rp: (&[u8; 32], &[u8]),
    pow: Option<&[u8; EXTENSION_BYTES]>,
) -> Option<Vec<u8>> {
    let (rp_onion_key, rp_specs) = rp;
    let mut cell = Vec::with_capacity(crate::cell::RELAY_BODY_BYTES);
    cell.extend_from_slice(&[0u8; 20]);
    cell.push(AUTH_KEY_ED25519);
    cell.extend_from_slice(&32u16.to_be_bytes());
    cell.extend_from_slice(auth);
    cell.push(0);

    let mut plain = Vec::with_capacity(INTRODUCE1_MIN);
    plain.extend_from_slice(cookie);
    match pow {
        Some(field) => {
            plain.push(1);
            plain.extend_from_slice(field);
        }
        None => plain.push(0),
    }
    plain.push(ONION_KEY_NTOR);
    plain.extend_from_slice(&32u16.to_be_bytes());
    plain.extend_from_slice(rp_onion_key);
    plain.extend_from_slice(rp_specs);
    if cell.len() + plain.len() < INTRODUCE1_MIN {
        plain.resize(INTRODUCE1_MIN - cell.len(), 0);
    }
    Ctr256Be::new(&keys.enc).apply(&mut plain);

    cell.extend_from_slice(client);
    cell.extend_from_slice(&plain);
    let mac = mac_sha3_256(&keys.mac, &cell);
    cell.extend_from_slice(&mac);
    (cell.len() <= crate::cell::RELAY_BODY_BYTES).then_some(cell)
}

/// The status INTRODUCE_ACK carries: 0 is success, anything else is the
/// introduction point's reason for not passing the request on.
pub fn introduce_ack(body: &[u8]) -> Option<u16> {
    let status = u16::from_be_bytes([*body.first()?, *body.get(1)?]);
    /* Then the extension count and extensions, which carry nothing a
     * client acts on; a body too short to hold the count is malformed. */
    body.get(2)?;
    Some(status)
}

/// RENDEZVOUS2's handshake: the service's Y and its auth MAC.
pub fn rendezvous2(body: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    if body.len() < 64 {
        return None;
    }
    Some((body[..32].try_into().ok()?, body[32..64].try_into().ok()?))
}

/// The link specifiers for a relay, count first: IPv4 and port, RSA
/// identity, Ed25519 identity. The same three EXTEND2 carries.
pub fn link_specifiers(address: [u8; 4], port: u16, rsa: &[u8; 20], ed: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + 8 + 22 + 34);
    out.push(3);
    out.extend_from_slice(&[0, 6]);
    out.extend_from_slice(&address);
    out.extend_from_slice(&port.to_be_bytes());
    out.extend_from_slice(&[2, 20]);
    out.extend_from_slice(rsa);
    out.extend_from_slice(&[3, 32]);
    out.extend_from_slice(ed);
    out
}

/// Whether a relay command is one of the replies a lookup waits for.
pub fn is_onion_reply(command: u8) -> bool {
    matches!(command, RELAY_RENDEZVOUS_ESTABLISHED | RELAY_INTRODUCE_ACK | RELAY_RENDEZVOUS2)
}

/// The BEGIN body for a stream to an onion service: an empty host, the
/// port, and no flags, as connection_ap_handshake_send_begin writes it for a
/// rendezvous circuit.
pub fn begin_onion(port: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    out.push(b':');
    let mut digits = [0u8; 5];
    let mut left = port;
    let mut count = 0usize;
    loop {
        digits[count] = b'0' + (left % 10) as u8;
        left /= 10;
        count += 1;
        if left == 0 {
            break;
        }
    }
    out.extend(digits[..count].iter().rev());
    out.push(0);
    out
}
