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


//! Blinding a public key by a scalar, for onion services.
//!
//! An onion service publishes under a different key each time period: its
//! identity key A multiplied by a factor h that both sides derive from A and
//! the period (rend-spec-v3 section 2.2.1). The client needs only the public
//! half, so this never touches a secret.

use crate::point::{ge_pack, ge_unpack, scalarmult_vartime};
use crate::scalar::{clamp_scalar, sc_reduce_mod_l};

/// `h * A`, where `h` is `param` clamped as an Ed25519 scalar is and reduced
/// mod l, as ed25519_donna_blind_public_key computes it. `None` when `public`
/// is not a point on the curve. Variable time is right here: both inputs are
/// public.
pub fn blind_public(public: &[u8; 32], param: &[u8; 32]) -> Option<[u8; 32]> {
    let point = ge_unpack(public)?;
    let mut clamped = *param;
    clamp_scalar(&mut clamped);
    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(&clamped);
    let h = sc_reduce_mod_l(&mut wide);
    Some(ge_pack(&scalarmult_vartime(&point, &h)))
}
