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

//! Signing for the proofs: the operator's own throwaway key, the command run
//! with it, and verify-signature run on what it wrote.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::cli::{CheckArgs, VerifyArgs};
use crate::plan::check_with;
use crate::sign::verify;
use crate::write::sig_path;
use ed25519_dalek::SigningKey;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// The operator's key, as its raw 32-byte seed in `dir`.
pub fn operator(dir: &Path, seed: u8) -> (PathBuf, SigningKey) {
    let path = dir.join("operator.seed");
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(&path, [seed; 32]).unwrap();
    (path, SigningKey::from_bytes(&[seed; 32]))
}

/// Check `s` signing with `key`; the exit status, and where the verdict went.
pub fn signed(s: &Sealed, key: &Path, json: bool, fake: Fake) -> (u8, TempDir, PathBuf) {
    let out = tempfile::tempdir().unwrap();
    let file = out.path().join("verdict");
    let (release, tree) = (s.release.path().into(), Some(s.tree.path().into()));
    let args =
        CheckArgs { release, tree, out: Some(file.clone()), sign_key: Some(key.into()), json };
    (check_with(&args, Box::new(fake)), out, file)
}

pub fn check_signature(verdict: &Path, public_key: &str) -> u8 {
    let (verdict, signature) = (verdict.to_path_buf(), sig_path(verdict));
    verify::run(&VerifyArgs { verdict, signature, public_key: public_key.into() })
}

pub fn public(key: &SigningKey) -> String {
    hex::encode(key.verifying_key().to_bytes())
}
