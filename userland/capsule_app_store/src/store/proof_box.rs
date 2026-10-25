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

//! The box a proof is asked against, from op 8's policy[256] | listing_key[32]
//! | release_key[32], checked before the prover is asked: the policy's
//! listing (policy[56..88], nonos_linux_policy's AT_LISTING) is zero or this
//! listing's key, the check capsule_prove/src/market/check.rs makes.
//!
//! Pure; market_proofs holds it.

use super::proof_fail::Fail;

pub const AT_LISTING: usize = 56;
/// policy | listing_key | release_key, market_proto's MARKET_ARG_LEN.
pub const ARG_LEN: usize = 320;

/// The policy and listing key a PROVE carries, or why none is asked:
/// NoBox for no policy (NONE, REFUSED or no answer), NotNamed for one
/// that names another listing.
pub fn judge(m: Option<&[u8; ARG_LEN]>) -> Result<([u8; 256], [u8; 32]), Fail> {
    let m = m.ok_or(Fail::NoBox)?;
    let mut policy = [0u8; 256];
    let mut key = [0u8; 32];
    policy.copy_from_slice(&m[..256]);
    key.copy_from_slice(&m[256..288]);
    let named = &policy[AT_LISTING..AT_LISTING + 32];
    if named != [0u8; 32] && named != key {
        return Err(Fail::NotNamed);
    }
    Ok((policy, key))
}
