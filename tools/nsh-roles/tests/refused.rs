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

//! What the tool refuses, writing nothing: a table signed by a key it does
//! not list, a wrong anchor, a role the table does not name, an escalation
//! over an hour, an approval from a key that is not an approver or that has
//! approved already, a key file that is not one, and any file that exists.

mod common;
#[path = "common/refuse.rs"]
mod refuse;
use common::{fleet, fresh, ok, FLEET};
use refuse::refused;
use std::os::unix::fs::PermissionsExt;

#[test]
fn keygen_is_owner_only_and_never_overwrites() {
    let d = fresh("keygen");
    ok(&d, &["keygen", "--out", ".", "--name", "auth"]);
    let mode = std::fs::metadata(d.join("auth.secret")).expect("secret").permissions().mode();
    assert_eq!(mode & 0o777, 0o600);
    assert_eq!(std::fs::read(d.join("auth.public")).expect("public").len(), 1984);
    refused(&d, &["keygen", "--out", ".", "--name", "auth"], "File exists", "auth.part");
    refused(&d, &["keygen", "--out", ".", "--name", "../x"], "--name", "x.secret");
}

#[test]
fn a_table_is_signed_only_by_a_key_it_lists() {
    let d = fresh("table");
    fleet(&d);
    let args = ["table", "--fleet", FLEET, "--serial", "2", "--max-age-hours", "24"];
    let rest = ["--authority", "auth.public", "--key", "other.secret", "--out", "t2"];
    refused(&d, &[&args[..], &rest].concat(), "Signer", "t2");
    let rest = ["--authority", "auth.public", "--key", "user.public", "--out", "t3"];
    refused(&d, &[&args[..], &rest].concat(), "key file", "t3");
}
