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


//! The client half of hs-ntor (the fork's src/core/crypto/hs_ntor.c).
//!
//! Pure: the two X25519 results are computed by the caller with the
//! ephemeral key it holds, so this file never sees a secret scalar.

use crate::crypto::equal;
use crate::crypto::keccak::{mac_sha3_256, Shake256};

const PROTOID: &[u8] = b"tor-hs-ntor-curve25519-sha3-256-1";
const T_HSENC: &[u8] = b"tor-hs-ntor-curve25519-sha3-256-1:hs_key_extract";
const T_HSVERIFY: &[u8] = b"tor-hs-ntor-curve25519-sha3-256-1:hs_verify";
const T_HSMAC: &[u8] = b"tor-hs-ntor-curve25519-sha3-256-1:hs_mac";
const M_HSEXPAND: &[u8] = b"tor-hs-ntor-curve25519-sha3-256-1:hs_key_expand";

/// Df, Db, Kf and Kb of the virtual hop: two SHA3-256 seeds and two
/// AES-256 keys (HS_NTOR_KEY_EXPANSION_KDF_OUT_LEN).
pub const HOP_KEY_BYTES: usize = 128;

/// The keys INTRODUCE1's encrypted part is sealed with.
pub struct IntroKeys {
    pub enc: [u8; 32],
    pub mac: [u8; 32],
}

impl Drop for IntroKeys {
    fn drop(&mut self) {
        wipe(&mut self.enc);
        wipe(&mut self.mac);
    }
}

/// hs_ntor_client_get_introduce1_keys, given `dh_bx` = EXP(B, x). `auth` is
/// the introduction point's auth key, `client` our X, `enc` the service's
/// B at that point.
pub fn intro_keys(dh_bx: &[u8; 32], auth: &[u8; 32], client: &[u8; 32], enc: &[u8; 32], subcredential: &[u8; 32]) -> IntroKeys {
    let mut kdf = Shake256::new();
    for part in [&dh_bx[..], auth, client, enc, PROTOID, T_HSENC, M_HSEXPAND, subcredential] {
        kdf.update(part);
    }
    let mut keys = IntroKeys { enc: [0u8; 32], mac: [0u8; 32] };
    kdf.squeeze(&mut keys.enc);
    kdf.squeeze(&mut keys.mac);
    keys
}

/// hs_ntor_client_get_rendezvous1_keys and the key expansion: the hop keys
/// when the service's `server_auth` from RENDEZVOUS2 matches, `None` when
/// it does not or either X25519 result is all zero.
pub fn rend_keys(
    dh_yx: &[u8; 32],
    dh_bx: &[u8; 32],
    auth: &[u8; 32],
    enc: &[u8; 32],
    client: &[u8; 32],
    server: &[u8; 32],
    server_auth: &[u8; 32],
) -> Option<[u8; HOP_KEY_BYTES]> {
    if dh_yx.iter().all(|b| *b == 0) || dh_bx.iter().all(|b| *b == 0) {
        return None;
    }
    let mut secret = [0u8; 32 * 6 + PROTOID.len()];
    for (at, part) in [dh_yx, dh_bx, auth, enc, client, server].iter().enumerate() {
        secret[at * 32..at * 32 + 32].copy_from_slice(&part[..]);
    }
    secret[192..].copy_from_slice(PROTOID);
    let mut seed = mac_sha3_256(&secret, T_HSENC);
    let verify = mac_sha3_256(&secret, T_HSVERIFY);
    wipe(&mut secret);

    let mut auth_input = [0u8; 32 * 5 + PROTOID.len() + 6];
    for (at, part) in [&verify, auth, enc, server, client].iter().enumerate() {
        auth_input[at * 32..at * 32 + 32].copy_from_slice(&part[..]);
    }
    auth_input[160..160 + PROTOID.len()].copy_from_slice(PROTOID);
    auth_input[160 + PROTOID.len()..].copy_from_slice(b"Server");
    let expected = mac_sha3_256(&auth_input, T_HSMAC);
    if !equal(&expected, server_auth) {
        wipe(&mut seed);
        return None;
    }
    let mut kdf = Shake256::new();
    kdf.update(&seed);
    kdf.update(M_HSEXPAND);
    wipe(&mut seed);
    let mut keys = [0u8; HOP_KEY_BYTES];
    kdf.squeeze(&mut keys);
    Some(keys)
}

fn wipe(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        /* SAFETY: eK@nonos.systems. Handshake secrets, wiped with a store
         * the optimiser may not remove. */
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
