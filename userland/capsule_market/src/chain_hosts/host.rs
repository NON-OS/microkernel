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

//! keccak, BLAKE3 and secp256k1 recovery for the commerce verdicts, in this
//! process from pure libraries. The market holds no Crypto capability and
//! needs none for this: hashing public bytes asks the kernel for nothing.

use nonos_marketplace_abi::{Blake3, Keccak, Recover};
use nonos_secp256k1::{recover_public_key, RecoverableSignature};
use sha3::{Digest, Keccak256};

pub(crate) struct Host;

impl Keccak for Host {
    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        Keccak256::digest(data).into()
    }
}

impl Blake3 for Host {
    fn blake3(&self, data: &[u8]) -> [u8; 32] {
        *blake3::hash(data).as_bytes()
    }
}

impl Recover for Host {
    fn signer(&self, digest: &[u8; 32], sig: &[u8; 65]) -> Option<[u8; 20]> {
        let recovery_id = sig[64].checked_sub(27)?;
        let mut rs = RecoverableSignature { r: [0; 32], s: [0; 32], recovery_id };
        rs.r.copy_from_slice(&sig[..32]);
        rs.s.copy_from_slice(&sig[32..64]);
        let pk = recover_public_key(digest, &rs)?;
        let mut who = [0u8; 20];
        who.copy_from_slice(&self.keccak256(&pk[1..])[12..]);
        Some(who)
    }
}
