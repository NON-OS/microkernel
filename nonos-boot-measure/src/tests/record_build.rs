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

//! Boot-root records signed with an independent P-256 implementation, so the
//! check is tested against a signer it shares no code with.

use p256::ecdsa::signature::hazmat::{PrehashSigner, PrehashVerifier};
use p256::ecdsa::{Signature, SigningKey};

use crate::record::message;

pub(super) fn key() -> SigningKey {
    SigningKey::from_bytes(&[7u8; 32].into()).expect("a scalar below n")
}

pub(super) fn record(root: [u8; 32], epoch: u64) -> Vec<u8> {
    let sig: Signature = key().sign_prehash(&message(&root, epoch)).expect("signs");
    let mut b = root.to_vec();
    b.extend(epoch.to_le_bytes());
    b.extend(sig.to_bytes());
    b
}

pub(super) fn verifier(d: &[u8; 32], r: &[u8; 32], s: &[u8; 32]) -> bool {
    let Ok(sig) = Signature::from_scalars(*r, *s) else { return false };
    key().verifying_key().verify_prehash(d, &sig).is_ok()
}
