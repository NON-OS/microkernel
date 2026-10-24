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

//! `verify-signature`: the verdict file is signed, byte for byte, by the key
//! given, and names that key as its signer. It says nothing of whether the
//! verdict is GO; it says the verdict is the one that key signed.

use super::named::{line, public_key, signer_named};
use crate::cli::VerifyArgs;
use crate::exit;
use crate::release::read::read_regular;
use crate::verdict::text::clean;
use ed25519_dalek::{Signature, VerifyingKey};
use std::path::Path;

pub fn run(a: &VerifyArgs) -> u8 {
    match verify(a) {
        Ok(line) => {
            println!("{}", clean(&line));
            exit::GO
        }
        Err(e) => {
            eprintln!("nonos-harden: {}", clean(&e));
            exit::SIGNATURE_BAD
        }
    }
}

fn verify(a: &VerifyArgs) -> Result<String, String> {
    let read = |p: &Path, max| read_regular(p, max).map_err(|e| format!("{}: {e}", p.display()));
    let verdict = read(&a.verdict, 64 << 20)?;
    let sig = String::from_utf8(read(&a.signature, 1024)?).unwrap_or_default();
    let mut s = [0u8; 64];
    hex::decode_to_slice(sig.trim(), &mut s)
        .map_err(|_| "the signature file is not 128 hex digits")?;
    let public = public_key(&a.public_key)?;
    let vk = VerifyingKey::from_bytes(&public).map_err(|_| "not an Ed25519 public key")?;
    vk.verify_strict(&verdict, &Signature::from_bytes(&s))
        .map_err(|_| "the signature does not verify: the verdict is not what this key signed")?;
    let text = String::from_utf8(verdict).map_err(|_| "the verdict is not text")?;
    let named = signer_named(&text).ok_or("the verdict names no signer")?;
    match named == hex::encode(public) {
        true => {
            Ok(format!("the signature verifies, by {named}; the verdict reads: {}", line(&text)))
        }
        false => Err(format!("signed by this key, but the verdict names {named} as its signer")),
    }
}
