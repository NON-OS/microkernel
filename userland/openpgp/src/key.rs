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

//! A v4 public key or subkey packet: its fingerprint and what it verifies
//! with. RSA, EdDSA over Ed25519, and native Ed25519; any other algorithm is
//! not a signing key here and is skipped by the keyring.

use alloc::vec::Vec;

use sha1::{Digest, Sha1};

use super::mpi::mpi;

/// 1.3.6.1.4.1.11591.15.1, the curve legacy EdDSA keys name.
const ED25519_OID: [u8; 9] = [0x2B, 0x06, 0x01, 0x04, 0x01, 0xDA, 0x47, 0x0F, 0x01];

#[derive(Clone)]
pub enum Material {
    Rsa { n: Vec<u8>, e: Vec<u8> },
    Ed25519([u8; 32]),
}

#[derive(Clone)]
pub struct Key {
    pub fingerprint: [u8; 20],
    pub material: Material,
}

pub fn key(body: &[u8]) -> Option<Key> {
    if *body.first()? != 4 {
        return None;
    }
    let rest = body.get(6..)?;
    let material = match *body.get(5)? {
        1 | 3 => {
            let (n, rest) = mpi(rest)?;
            let (e, _) = mpi(rest)?;
            Material::Rsa { n: n.to_vec(), e: e.to_vec() }
        }
        22 => {
            let len = usize::from(*rest.first()?);
            if rest.get(1..1 + len)? != ED25519_OID {
                return None;
            }
            let (point, _) = mpi(&rest[1 + len..])?;
            match point {
                [0x40, pk @ ..] => Material::Ed25519(pk.try_into().ok()?),
                _ => return None,
            }
        }
        27 => Material::Ed25519(rest.get(..32)?.try_into().ok()?),
        _ => return None,
    };
    let len = u16::try_from(body.len()).ok()?;
    let mut h = Sha1::new();
    h.update([0x99]);
    h.update(len.to_be_bytes());
    h.update(body);
    Some(Key { fingerprint: h.finalize().into(), material })
}
