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

//! The command line is read strictly: a flag unknown, repeated or without a
//! value is refused, so a typo can never run a weaker check than was asked.

use crate::cli::{parse, CheckArgs, Command, VerifyArgs};

fn line(s: &str) -> Result<Command, String> {
    parse(&s.split_whitespace().map(String::from).collect::<Vec<_>>())
}

#[test]
fn a_full_command_is_read_as_given() {
    let c = line("check --release r --tree t --format json --out o --sign-key k");
    let (tree, out, sign_key) = (Some("t".into()), Some("o".into()), Some("k".into()));
    let want = CheckArgs { release: "r".into(), tree, out, sign_key, json: true };
    assert_eq!(c, Ok(Command::Check(want)));
    let v = line("verify-signature --verdict v --signature s --public-key p");
    let want = VerifyArgs { verdict: "v".into(), signature: "s".into(), public_key: "p".into() };
    assert_eq!(v, Ok(Command::VerifySignature(want)));
    assert_eq!(line("--help"), Ok(Command::Help));
    assert_eq!(line("--version"), Ok(Command::Version));
}

#[test]
fn anything_else_is_refused() {
    for bad in [
        "",
        "check",
        "check --release",
        "check --release --tree t",
        "check --release r --release r",
        "check --release r --skip release.sealed",
        "check --release r --format yaml",
        "check --release r extra",
        "check --release r --sign-key k",
        "--help check",
        "verify",
        "verify-signature --verdict v --signature s",
        "verify-signature --verdict v --signature s --public-key p --x y",
    ] {
        assert!(line(bad).is_err(), "{bad:?} was accepted");
    }
    let empty = parse(&["check".into(), "--release".into(), String::new()]);
    assert!(empty.is_err(), "an empty value was accepted");
}
