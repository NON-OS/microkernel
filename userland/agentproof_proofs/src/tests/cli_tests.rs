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

//! What `agentproof` prints and exits with, through cli.rs, which the binary
//! only reads a file for and prints.

use agentproof_policy::cli::{answer, ask, unreadable, usage, Ask, USAGE};

use super::file;

const STRICT: &str = "5811b081f826f154ce1ac5a7b558bff858ae0bacc27ad2db9d2e5f84ce05323a";

#[test]
fn the_words_it_takes() {
    assert!(matches!(ask(&["policy", "check", "a.box"]), Ask::Policy("check", "a.box")));
    assert!(matches!(ask(&["policy", "word", "a"]), Ask::Policy("word", "a")));
    assert!(matches!(ask(&["policy", "hash", "a"]), Ask::Policy("hash", "a")));
    assert!(matches!(ask(&["--help"]), Ask::Help));
    assert!(matches!(ask(&["help"]), Ask::Help));
    assert!(matches!(ask(&["--version"]), Ask::Version));
    for wrong in [&[][..], &["policy"], &["policy", "check"], &["policy", "lint", "a"]] {
        assert!(matches!(ask(wrong), Ask::Usage));
    }
    assert!(matches!(ask(&["policy", "check", "a", "b"]), Ask::Usage));
    let u = usage();
    assert_eq!((u.code, u.err[0].as_str()), (2, USAGE));
}

#[test]
fn a_policy_is_explained_with_its_word_and_hash() {
    let strict = file(&["nonos-box 1", "base strict", "network none"]);
    let a = answer("check", "s.box", &strict);
    assert_eq!(a.code, 0);
    assert!(a.err.is_empty());
    assert_eq!(a.out[0], format!("policy   strict {STRICT}"));
    assert_eq!(a.out[1], "run      linux --box strict <program> [arguments]");
    assert_eq!(a.out[2], "allowed  write /tmp");
    assert!(a.out.iter().any(|l| l == "It may write only inside /tmp."));
    assert!(a.out.iter().any(|l| l.starts_with("It has no network.")));
    assert_eq!(answer("hash", "s.box", &strict).out, [STRICT]);
    assert_eq!(answer("word", "s.box", &strict).out, ["strict"]);
}

#[test]
fn a_refusal_names_the_file_and_line() {
    let bad = file(&["nonos-box 1", "base strict", "network any"]);
    let a = answer("check", "bad.box", &bad);
    assert_eq!(a.code, 1);
    assert!(a.out.is_empty());
    let why = "a terminal run has no network in this release";
    assert_eq!(a.err, [format!("agentproof: refused: bad.box:3: {why}")]);
    let whole = answer("word", "w.box", &file(&["nonos-box 1", "base strict"]));
    let missing = "network is missing; write `network none`";
    assert_eq!(whole.err, [format!("agentproof: refused: w.box: {missing}")]);
}

#[test]
fn a_file_it_cannot_read_is_its_own_exit() {
    let a = unreadable("gone.box", "No such file or directory (os error 2)");
    assert_eq!((a.code, a.out.len()), (2, 0));
    assert_eq!(a.err, ["agentproof: gone.box: No such file or directory (os error 2)"]);
}

#[test]
fn a_folders_word_is_sorted_and_once() {
    let a = file(&["nonos-box 1", "base folders", "write /tmp/b", "write /home/a", "network none"]);
    assert_eq!(answer("word", "a", &a).out, ["allow:/home/a,/tmp/b"]);
}
