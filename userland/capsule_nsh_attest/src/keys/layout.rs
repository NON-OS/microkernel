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

//! The host keys as they are sealed: the Ed25519 seed, the ML-DSA-65
//! secret key, then its public key, fixed lengths and nothing else. The
//! public key is kept beside the secret because ML-DSA-65's secret key
//! does not give it back. Pure, so nsh_front_proofs holds it.

/// The Ed25519 seed's length.
pub const ED_SEED_LEN: usize = 32;
/// The ML-DSA-65 secret key's length.
pub const ML_SECRET_LEN: usize = 4032;
/// The ML-DSA-65 public key's length.
pub const ML_PUBLIC_LEN: usize = 1952;
/// The whole plaintext.
pub const PLAIN_LEN: usize = ED_SEED_LEN + ML_SECRET_LEN + ML_PUBLIC_LEN;

// The three lengths fill the plaintext exactly, so each chunk `split`
// takes is there and its fallbacks are never reached.
const _: () = assert!(ED_SEED_LEN + ML_SECRET_LEN + ML_PUBLIC_LEN == PLAIN_LEN);

const ML_SECRET_AT: usize = ED_SEED_LEN;
const ML_PUBLIC_AT: usize = ML_SECRET_AT + ML_SECRET_LEN;

/// The three parts, laid out in order.
pub fn join(
    seed: &[u8; ED_SEED_LEN],
    secret: &[u8; ML_SECRET_LEN],
    public: &[u8; ML_PUBLIC_LEN],
    out: &mut [u8; PLAIN_LEN],
) {
    out[..ML_SECRET_AT].copy_from_slice(seed);
    out[ML_SECRET_AT..ML_PUBLIC_AT].copy_from_slice(secret);
    out[ML_PUBLIC_AT..].copy_from_slice(public);
}

/// The three parts of a plaintext `join` laid out.
pub fn split(
    plain: &[u8; PLAIN_LEN],
) -> (&[u8; ED_SEED_LEN], &[u8; ML_SECRET_LEN], &[u8; ML_PUBLIC_LEN]) {
    let (seed, rest) = plain.split_first_chunk::<ED_SEED_LEN>().unwrap_or((&[0; ED_SEED_LEN], &[]));
    let (secret, public) =
        rest.split_first_chunk::<ML_SECRET_LEN>().unwrap_or((&[0; ML_SECRET_LEN], &[]));
    let public = public.first_chunk::<ML_PUBLIC_LEN>().unwrap_or(&[0; ML_PUBLIC_LEN]);
    (seed, secret, public)
}
