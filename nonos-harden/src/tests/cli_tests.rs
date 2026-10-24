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

use crate::cli::{parse, CheckArgs, Command};

fn args(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

#[test]
fn a_full_command_is_read_as_given() {
    let c =
        parse(&args(&["check", "--release", "r", "--tree", "t", "--format", "json", "--out", "o"]));
    let want = CheckArgs {
        release: "r".into(),
        tree: Some("t".into()),
        out: Some("o".into()),
        json: true,
    };
    assert_eq!(c, Ok(Command::Check(want)));
    assert_eq!(parse(&args(&["--help"])), Ok(Command::Help));
    assert_eq!(parse(&args(&["--version"])), Ok(Command::Version));
}

#[test]
fn anything_else_is_refused() {
    for bad in [
        &[][..],
        &["check"],
        &["check", "--release"],
        &["check", "--release", ""],
        &["check", "--release", "--tree", "t"],
        &["check", "--release", "r", "--release", "r"],
        &["check", "--release", "r", "--skip", "release.sealed"],
        &["check", "--release", "r", "--format", "yaml"],
        &["check", "--release", "r", "extra"],
        &["--help", "check"],
        &["verify"],
    ] {
        assert!(parse(&args(bad)).is_err(), "{bad:?} was accepted");
    }
}
