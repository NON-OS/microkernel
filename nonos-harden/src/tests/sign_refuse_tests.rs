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

//! A NONOS key is never used to sign: one lying where NONOS keeps its keys,
//! reached through a link or not, is refused before any check runs, and
//! nothing is written.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use super::signing::{operator, signed};
use crate::exit;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn refused(s: &Sealed, key: &Path) {
    let fake = Fake::default();
    let ran = fake.ran.clone();
    let (code, _out, file) = signed(s, key, true, fake);
    assert_eq!(code, exit::USAGE, "{} was used", key.display());
    assert!(ran.borrow().is_empty() && !file.exists(), "a check ran, or a verdict was written");
}

/// A directory laid out as a NONOS checkout is, as far as the refusal looks.
fn checkout() -> tempfile::TempDir {
    let c = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(c.path().join("nonos-bootloader")).unwrap();
    std::fs::create_dir_all(c.path().join("nonos-data")).unwrap();
    c
}

#[test]
fn a_key_where_nonos_keeps_its_keys_is_refused_before_any_check_runs() {
    let (s, c) = (Sealed::dev(), checkout());
    for at in [".keys", "nonos-bootloader/keys", "nonos-data/trust/keys"] {
        refused(&s, &operator(&c.path().join(at), 42).0);
    }
    let elsewhere = tempfile::tempdir().unwrap();
    refused(&s, &operator(&elsewhere.path().join("x/nonos-bootloader/keys"), 42).0);
}

#[test]
fn a_link_to_a_nonos_key_is_refused_by_where_it_points() {
    let (s, c, neutral) = (Sealed::dev(), checkout(), tempfile::tempdir().unwrap());
    let (real, _) = operator(&c.path().join(".keys"), 42);
    let link = neutral.path().join("my.seed");
    symlink(&real, &link).unwrap();
    refused(&s, &link);
}
