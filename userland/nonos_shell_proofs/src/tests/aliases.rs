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
//! A session's aliases and `type`: an alias replaces a command's first word
//! as it runs, once, held to the same bounds as a variable; `type` says
//! what a name runs, in the order the session looks.

use crate::kit::{Lines, Script};
use nonos_shell::session::{Session, ALIAS_REFUSED, MAX_ALIASES, MAX_VALUE, NAMES};

fn run(s: &mut Session, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    s.run_line(line.as_bytes(), &mut out, &mut Script::default());
    out.text()
}

#[test]
fn an_alias_replaces_the_first_word_and_keeps_the_rest() {
    let mut s = Session::default();
    assert_eq!(run(&mut s, "alias"), ["(no aliases)"]);
    assert_eq!(run(&mut s, "alias say echo said; echo $?"), ["0"]);
    assert_eq!(run(&mut s, "say a b"), ["said a b"]);
    assert_eq!(run(&mut s, "alias"), ["say = echo said"]);
    assert_eq!(run(&mut s, "alias say echo again; say"), ["again"]);
    assert_eq!(run(&mut s, "unalias say; say"), ["say: command not found"]);
    assert_eq!(run(&mut s, "unalias say; echo $?"), ["0"], "removing what is not set succeeds");
}

#[test]
fn an_alias_is_read_when_it_runs_and_through_the_same_checks() {
    let mut s = Session::default();
    run(&mut s, "alias n 'seq 3 | wc -l'");
    assert_eq!(run(&mut s, "n"), ["lines 3"]);
    run(&mut s, "alias v 'echo $X'");
    assert_eq!(run(&mut s, "X=1; v; X=2; v"), ["1", "2"]);
    run(&mut s, "alias w 'echo a > f'");
    assert_eq!(run(&mut s, "w"), ["redirects are not available in this shell"]);
    run(&mut s, "alias j 'echo a; echo b'");
    assert_eq!(run(&mut s, "j"), ["a; echo b"], "an alias body is one command, never two");
}

#[test]
fn an_alias_is_expanded_once_so_none_loops() {
    let mut s = Session::default();
    run(&mut s, "alias a b; alias b a");
    assert_eq!(run(&mut s, "a"), ["b: command not found"]);
    run(&mut s, "alias echo echo x");
    assert_eq!(run(&mut s, "echo y"), ["x y"]);
}

#[test]
fn alias_and_unalias_refuse_what_they_cannot_read() {
    let mut s = Session::default();
    assert_eq!(run(&mut s, "alias say"), ["usage: alias [NAME WORD...]"]);
    assert_eq!(run(&mut s, "alias 9x echo; echo $?"), ["usage: alias [NAME WORD...]", "1"]);
    assert_eq!(run(&mut s, "alias a=b echo"), ["usage: alias [NAME WORD...]"]);
    assert_eq!(run(&mut s, "unalias"), ["usage: unalias NAME"]);
    assert_eq!(run(&mut s, "unalias a b"), ["usage: unalias NAME"]);
    assert!(s.aliases.is_empty());
}

#[test]
fn aliases_are_held_to_the_bounds() {
    let mut s = Session::default();
    let fits = "x".repeat(MAX_VALUE);
    let long = "x".repeat(MAX_VALUE + 1);
    assert!(s.set_alias(b"FITS", fits.as_bytes()));
    assert_eq!(
        run(&mut s, &format!("alias LONG {long}; echo $?")),
        [String::from_utf8_lossy(ALIAS_REFUSED).into_owned(), "1".into()]
    );
    for i in 1..MAX_ALIASES {
        assert!(s.set_alias(format!("A{i}").as_bytes(), b"echo"), "{i}");
    }
    assert!(!s.set_alias(b"ONE_MORE", b"echo"));
    assert_eq!(run(&mut s, "alias ONE_MORE echo; echo $?").last().map(String::as_str), Some("1"));
    assert!(s.set_alias(b"A1", b"echo changed"), "an existing name may still change");
    assert_eq!(s.aliases.len(), MAX_ALIASES);
    assert!(s.aliases.iter().all(|(_, v)| v.len() <= MAX_VALUE));
}

#[test]
fn type_says_what_each_name_runs() {
    let mut s = Session::default();
    run(&mut s, "alias ll echo long; alias seq echo mine");
    assert_eq!(
        run(&mut s, "type ll seq echo for grep nope; echo $?"),
        [
            "ll is an alias for echo long",
            "seq is an alias for echo mine",
            "echo is built into the shell",
            "for is a control word",
            "grep is a filter, run after |",
            "nope not found",
            "1",
        ]
    );
    assert_eq!(run(&mut s, "which echo; echo $?"), ["echo is built into the shell", "0"]);
    assert_eq!(run(&mut s, "type"), ["usage: type NAME..."]);
    let mut s = Session::default();
    for name in NAMES {
        let name = String::from_utf8_lossy(name);
        assert_eq!(
            run(&mut s, &format!("type {name}")),
            [format!("{name} is built into the shell")]
        );
    }
}

#[test]
fn help_lists_every_name_type_knows() {
    let mut s = Session::default();
    let help = run(&mut s, "help");
    for name in ["alias", "unalias", "type", "which"] {
        assert!(help[0].split(' ').any(|w| w == name), "{name} in {help:?}");
    }
}
