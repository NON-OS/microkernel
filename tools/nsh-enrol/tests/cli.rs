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

//! The command line, run whole: a new key is written owner-only and named
//! by its id, keygen never writes over a key, and wrong arguments are
//! refused.

use std::os::unix::fs::PermissionsExt;

mod common;
use common::{fresh, run, said};

#[test]
fn keygen_writes_an_owner_only_secret_and_never_overwrites() {
    let d = fresh("keygen");
    let o = run(&d, &["keygen", "--out", "."]);
    assert!(o.status.success(), "{}", said(&o));
    let id = said(&o).trim().strip_prefix("authority key id ").expect("an id").to_string();
    let public = std::fs::read(d.join("authority.public")).expect("public");
    assert_eq!(public.len(), 1984);
    assert_eq!(id, nsh_enrol::hex::of(&nonos_hash::sha256(&public)));
    let mode = std::fs::metadata(d.join("authority.secret")).expect("secret").permissions().mode();
    assert_eq!(mode & 0o777, 0o600);
    let again = run(&d, &["keygen", "--out", "."]);
    assert!(!again.status.success() && said(&again).contains("already there"), "{}", said(&again));
    assert_eq!(std::fs::read(d.join("authority.public")).expect("kept"), public);
}

#[test]
fn wrong_arguments_are_refused() {
    let d = fresh("args");
    for args in [&[][..], &["frob"], &["keygen"], &["keygen", "--out"], &["keygen", "out"]] {
        let o = run(&d, args);
        assert!(!o.status.success(), "{args:?}");
        assert!(said(&o).starts_with("nsh-enrol: "), "{args:?}: {}", said(&o));
    }
    let o = run(&d, &["keygen", "--out", ".", "--out", "."]);
    assert!(said(&o).contains("more than once"), "{}", said(&o));
}
