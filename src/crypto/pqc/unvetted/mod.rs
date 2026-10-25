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

//! SPHINCS+, NTRU and Classic McEliece, which do not ship. Each of the
//! kernel's own implementations fails its first full run: NTRU's key
//! generation never ends, SPHINCS+ signing shifts a 32-bit leaf index past
//! its width, and McEliece's key generation panics in `polynomial_gcd`.
//! They are built only under the `pqc-unvetted` feature, which no default
//! or release profile enables, until each passes its published vectors.

extern crate alloc;

mod mceliece;
mod ntru;
mod sphincs;

pub use mceliece::*;
pub use ntru::*;
pub use sphincs::*;

pub use super::sphincs::{
    sphincs_keygen, sphincs_sign, sphincs_verify, SphincsKeyPair, SphincsPublicKey,
    SphincsSecretKey, SphincsSignature, SPHINCS_PK_BYTES, SPHINCS_SIG_BYTES, SPHINCS_SK_BYTES,
};

pub use super::ntru::{
    ntru_decaps, ntru_encaps, ntru_keygen, NtruCiphertext, NtruKeyPair, NtruPublicKey,
    NtruSecretKey, NTRU_CIPHERTEXT_BYTES, NTRU_PUBLICKEY_BYTES, NTRU_SECRETKEY_BYTES,
    NTRU_SHARED_SECRET_BYTES,
};

pub use super::mceliece::{
    mceliece_decaps, mceliece_encaps, mceliece_keygen, McElieceCiphertext, McElieceKeyPair,
    McEliecePublicKey, McElieceSecretKey, MCELIECE_CIPHERTEXT_BYTES, MCELIECE_PUBLICKEY_BYTES,
    MCELIECE_SECRETKEY_BYTES, MCELIECE_SHARED_SECRET_BYTES,
};

