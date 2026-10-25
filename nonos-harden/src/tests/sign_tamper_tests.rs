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

//! A verdict that names another signer does not verify, though its signature
//! does; and a signature that cannot be written leaves no unsigned verdict.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use super::signing::{check_signature, operator, public, signed};
use crate::exit;
use crate::write::sig_path;
use std::fs;

#[test]
fn a_verdict_naming_another_signer_does_not_verify_though_signed() {
    let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
    let (path, key) = operator(dir.path(), 42);
    let (_, _out, file) = signed(&s, &path, true, Fake::default());
    let other = public(&operator(&dir.path().join("other"), 43).1);
    let renamed = fs::read_to_string(&file).unwrap().replace(&public(&key), &other);
    fs::write(&file, &renamed).unwrap();
    fs::write(sig_path(&file), crate::sign::sign(&key, renamed.as_bytes())).unwrap();
    assert_eq!(check_signature(&file, &public(&key)), exit::SIGNATURE_BAD);
}

#[test]
fn a_signature_that_cannot_be_written_leaves_no_unsigned_verdict() {
    let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
    let (path, _) = operator(dir.path(), 42);
    let out = tempfile::tempdir().unwrap();
    let file = out.path().join("verdict");
    fs::create_dir(sig_path(&file)).unwrap();
    let (release, tree) = (s.release.path().into(), Some(s.tree.path().into()));
    let args = crate::cli::CheckArgs {
        release,
        tree,
        out: Some(file.clone()),
        sign_key: Some(path),
        json: true,
    };
    assert_eq!(crate::plan::check_with(&args, Box::new(Fake::default())), exit::NOT_DELIVERED);
    assert!(!file.exists(), "an unsigned verdict was left where a signed one was asked for");
}
