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

//! A FETCH 1 request's proof of work (plan/29, Tor's rule): the solution
//! field is nonce[16] | effort u32 BE | Equi-X solution[16]; the challenge is
//! listing_key | buyer_pk | nonce | effort (84 bytes). It passes when Equi-X verifies
//! over the challenge and BLAKE2b-32(challenge | solution), read big endian,
//! times effort fits a u32, with effort at least the agent's least. The
//! meter's tests and B5b's agent both check with `pow_check`.

use nonos_equix::{verify, Blake2b, Solution, SOLUTION_BYTES};

use crate::pow_field::PowField;

/// listing_key | buyer_pk | nonce | effort: 32 + 32 + 16 + 4 (plan/29).
pub const CHALLENGE_LEN: usize = 84;
pub fn pow_challenge(
    listing_key: &[u8; 32],
    buyer_pk: &[u8; 32],
    nonce: &[u8; 16],
    effort: u32,
) -> [u8; CHALLENGE_LEN] {
    let mut c = [0u8; CHALLENGE_LEN];
    c[..32].copy_from_slice(listing_key);
    c[32..64].copy_from_slice(buyer_pk);
    c[64..80].copy_from_slice(nonce);
    c[80..].copy_from_slice(&effort.to_be_bytes());
    c
}

/// BLAKE2b-32(challenge | solution), big endian, times effort, fits a u32.
pub fn pow_ok(challenge: &[u8], solution: &[u8; SOLUTION_BYTES], effort: u32) -> bool {
    let mut hash = Blake2b::new(4, &[0; 16]);
    hash.update(challenge);
    hash.update(solution);
    let mut r = [0u8; 4];
    hash.finish(&mut r);
    let scaled = u64::from(u32::from_be_bytes(r)).checked_mul(u64::from(effort));
    scaled.is_some_and(|s| s <= u64::from(u32::MAX))
}

/// The whole check: the field parses, its effort is at least `least`, Equi-X
/// verifies over its challenge and pow_ok holds.
pub fn pow_check(listing_key: &[u8; 32], buyer_pk: &[u8; 32], field: &[u8], least: u32) -> bool {
    let Some(f) = PowField::parse(field).filter(|f| f.effort >= least) else { return false };
    let c = pow_challenge(listing_key, buyer_pk, &f.nonce, f.effort);
    verify(&c, &Solution::from_bytes(&f.solution)).is_ok() && pow_ok(&c, &f.solution, f.effort)
}
