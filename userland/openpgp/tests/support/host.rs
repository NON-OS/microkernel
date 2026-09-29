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

//! The test's `Verifier`: Ed25519 through nonos_ed25519, RSA by raising the
//! signature to e and comparing the whole PKCS#1 v1.5 block.

use nonos_ed25519::{verify, Signature};
use nonos_openpgp::Verifier;

use crate::modpow::modpow;

const SHA256_INFO: [u8; 19] = [
    0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05,
    0x00, 0x04, 0x20,
];
const SHA512_INFO: [u8; 19] = [
    0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05,
    0x00, 0x04, 0x40,
];

pub struct Host;

impl Verifier for Host {
    fn rsa(&self, n: &[u8], e: &[u8], sig: &[u8], hash: u8, digest: &[u8]) -> bool {
        let info: &[u8] = match hash {
            8 => &SHA256_INFO,
            10 => &SHA512_INFO,
            _ => return false,
        };
        let m = modpow(sig, e, n);
        let em = &m[m.len() - n.len()..];
        let pad = n.len() - 3 - info.len() - digest.len();
        let mut want = vec![0x00, 0x01];
        want.extend(std::iter::repeat_n(0xFF, pad));
        want.push(0x00);
        want.extend_from_slice(info);
        want.extend_from_slice(digest);
        m[..m.len() - n.len()].iter().all(|&b| b == 0) && em == want.as_slice()
    }

    fn ed25519(&self, key: &[u8; 32], sig: &[u8; 64], digest: &[u8]) -> bool {
        verify(key, digest, &Signature::from_bytes(sig))
    }
}
