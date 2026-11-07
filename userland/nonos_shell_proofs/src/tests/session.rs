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

//! A headless session runs a typed line as the terminal's language reads
//! it: statements gated on status, control flow read before anything runs,
//! names expanded as each command runs, pipes through the shared filters,
//! and every line bounded.

use crate::kit::{Lines, Script};
use nonos_shell::session::{Flow, Session, MAX_ROUNDS};

fn run(s: &mut Session, line: &str) -> (Flow, Vec<String>) {
    let mut out = Lines::default();
    let flow = s.run_line(line.as_bytes(), &mut out, &mut Script::default());
    (flow, out.text())
}

fn lines(line: &str) -> Vec<String> {
    run(&mut Session::default(), line).1
}

#[test]
fn statements_run_in_order_gated_on_status() {
    assert_eq!(lines("echo a; echo b"), ["a", "b"]);
    assert_eq!(lines("false && echo no || echo yes"), ["yes"]);
    assert_eq!(lines("true || echo no; echo $?"), ["0"]);
    assert_eq!(lines("false; echo $?"), ["1"]);
}

#[test]
fn if_takes_one_branch() {
    assert_eq!(lines("if true; then echo y; else echo n; fi"), ["y"]);
    assert_eq!(lines("if false; then echo y; else echo n; fi"), ["n"]);
    assert_eq!(lines("if true && false; then echo y; else echo n; fi"), ["n"]);
    let mut s = Session::default();
    assert_eq!(run(&mut s, "if false; then echo y; fi").1, Vec::<String>::new());
    assert_eq!(s.last_status, 0, "no branch taken is success");
    assert_eq!(
        lines("if test 3 -lt 5; then if test a = b; then echo 1; else echo 2; fi; fi"),
        ["2"]
    );
}

#[test]
fn for_reads_its_name_afresh_each_round() {
    assert_eq!(lines("for i in 1 2 3; do echo n$i; done"), ["n1", "n2", "n3"]);
    assert_eq!(
        lines("for i in 1 2; do for j in a b; do echo $i$j; done; done"),
        ["1a", "1b", "2a", "2b"]
    );
    let mut s = Session::default();
    run(&mut s, "set W x y");
    assert_eq!(
        run(&mut s, "for w in $W; do echo <$w>; done").1,
        ["redirects are not available in this shell"; 2]
    );
    assert_eq!(run(&mut s, "for w in $W; do echo [$w]; done").1, ["[x]", "[y]"]);
}

#[test]
fn while_runs_until_its_condition_fails() {
    let mut s = Session::default();
    let (_, out) = run(&mut s, "x=go; while test $x = go; do echo round; x=stop; done; echo $x");
    assert_eq!(out, ["round", "stop"]);
}

#[test]
fn a_while_that_never_ends_is_stopped() {
    let mut s = Session::default();
    let (flow, out) = run(&mut s, "while true; do true; done");
    assert_eq!(flow, Flow::Next);
    assert_eq!(out, [format!("while: stopped after {MAX_ROUNDS} rounds")]);
    assert_eq!(s.last_status, 1);
}

#[test]
fn a_line_is_held_to_its_steps() {
    let words = vec!["w"; 64].join(" ");
    let line = format!(
        "for a in {words}; do for b in {words}; do for c in {words}; do true; done; done; done"
    );
    let mut s = Session::default();
    let (_, out) = run(&mut s, &line);
    assert_eq!(out, ["shell: stopped after 100000 commands on one line"]);
}

#[test]
fn pipes_pass_the_shared_filters() {
    assert_eq!(lines("seq 5 | tail -n 2"), ["4", "5"]);
    assert_eq!(lines("seq 12 | grep 1 | wc -l"), ["lines 4"]);
}

#[test]
fn malformed_control_flow_runs_nothing() {
    for bad in [
        "if true; then echo x",
        "fi",
        "echo a; done",
        "for x; do echo; done",
        "for 1x in a; do echo; done",
        "while true; echo; done",
    ] {
        let mut s = Session::default();
        let (_, out) = run(&mut s, bad);
        assert_eq!(out.len(), 1, "{bad}: {out:?}");
        assert!(out[0].starts_with("syntax:"), "{bad}: {out:?}");
        assert_eq!(s.last_status, 2, "{bad}");
    }
    assert_eq!(lines("echo a &"), ["background jobs are not available in this shell"]);
    let deep = "if true; then ".repeat(17) + &"fi; ".repeat(17);
    assert_eq!(lines(&deep), ["syntax: blocks nested too deep"]);
}

#[test]
fn exit_ends_the_session_and_runs_nothing_after() {
    assert_eq!(
        run(&mut Session::default(), "echo a; exit 3; echo b"),
        (Flow::Exit(3), vec!["a".into()])
    );
    assert_eq!(run(&mut Session::default(), "for i in 1 2; do exit; done").0, Flow::Exit(0));
    assert_eq!(run(&mut Session::default(), "exit x").0, Flow::Next);
}

#[test]
fn names_and_aliases_expand_as_the_command_runs() {
    let mut s = Session::default();
    s.aliases.push((b"hi".to_vec(), b"echo hello".to_vec()));
    assert_eq!(run(&mut s, "hi nsh").1, ["hello nsh"]);
    assert_eq!(run(&mut s, "N=42; echo nsh-$N").1, ["nsh-42"]);
    assert_eq!(s.var(b"N"), Some(&b"42"[..]));
    run(&mut s, "unset N");
    assert_eq!(s.var(b"N"), None);
    let (_, out) = run(&mut s, "nosuch thing; echo $?");
    assert_eq!(out, ["nosuch: command not found", "127"]);
}

#[test]
fn test_compares_text_and_numbers() {
    assert_eq!(lines("test a = a && echo t"), ["t"]);
    assert_eq!(lines("test 10 -gt 9 && test -9 -lt 0 && test -z '' && echo t"), ["t"]);
    assert_eq!(lines("test 1 -lt x; echo $?").last().map(String::as_str), Some("2"));
}

#[test]
fn crlf_ends_each_line_for_a_remote_terminal() {
    let mut out = nonos_shell::out::Crlf::default();
    Session::default().run_line(b"echo a; echo", &mut out, &mut Script::default());
    assert_eq!(out.0, b"a\r\n\r\n");
}

#[test]
fn a_line_that_prints_without_end_is_stopped_at_the_output_bound() {
    use nonos_shell::out::{Crlf, CRLF_FULL, CRLF_MAX};
    let mut out = Crlf::default();
    let mut s = Session::default();
    let flow = s.run_line(
        b"while true; do seq 100000; done; echo after",
        &mut out,
        &mut Script::default(),
    );
    assert_eq!(flow, Flow::Next);
    assert!(out.0.len() <= CRLF_MAX + CRLF_FULL.len());
    assert!(out.0.ends_with(CRLF_FULL), "the bound is said once, last");
    assert_eq!(out.0.windows(CRLF_FULL.len()).filter(|w| *w == CRLF_FULL).count(), 1);
    assert_ne!(s.last_status, 0);
    let mut next = Crlf::default();
    s.run_line(b"echo next", &mut next, &mut Script::default());
    assert_eq!(next.0, b"next\r\n", "a fresh sink takes the next line whole");
}

#[test]
fn output_just_under_the_bound_is_kept_whole() {
    use nonos_shell::out::{Crlf, ShellOut, CRLF_MAX};
    let mut out = Crlf::default();
    let line = vec![b'x'; CRLF_MAX - 2];
    out.writeln(&line);
    assert!(!out.full());
    assert_eq!(out.0.len(), CRLF_MAX);
    out.writeln(b"");
    assert!(out.full());
    assert!(out.0.starts_with(&line));
}

#[test]
fn a_variable_that_doubles_each_round_is_stopped_at_its_bound() {
    use nonos_shell::session::{MAX_VALUE, VAR_REFUSED};
    let mut s = Session::default();
    let (_, out) = run(&mut s, "set A x; while true; do set A $A$A; done");
    assert!(out.iter().any(|l| l.as_bytes() == VAR_REFUSED));
    let a = s.var(b"A").unwrap();
    assert!(
        a.len() <= MAX_VALUE && a.len() * 2 > MAX_VALUE,
        "kept its last value within the bound"
    );
    assert_ne!(s.last_status, 0);
}

#[test]
fn every_way_to_set_a_variable_is_held_to_the_bounds() {
    use nonos_shell::session::{MAX_VALUE, MAX_VARS};
    let long = "x".repeat(MAX_VALUE + 1);
    let fits = "x".repeat(MAX_VALUE);
    let mut s = Session::default();
    assert!(s.set_var(b"FITS", fits.as_bytes()));
    assert!(!s.set_var(b"LONG", long.as_bytes()));
    run(&mut s, &format!("B={long}; echo $?"));
    run(&mut s, &format!("for C in {long}; do echo in; done"));
    assert_eq!((s.var(b"B"), s.var(b"C"), s.var(b"LONG")), (None, None, None));
    for i in 1..MAX_VARS {
        assert!(s.set_var(format!("V{i}").as_bytes(), b"v"), "{i}");
    }
    assert!(!s.set_var(b"ONE_MORE", b"v"));
    assert!(s.set_var(b"V1", b"changed"), "an existing name may still change");
    let (_, out) = run(&mut s, "set ONE_MORE v; echo $?");
    assert_eq!(out.last().map(String::as_str), Some("1"));
}

#[test]
fn loops_with_empty_bodies_are_held_to_the_steps() {
    // 64^3 rounds with no command in them is over MAX_STEPS.
    let words = vec!["w"; 64].join(" ");
    let line = format!(
        "for a in {words}; do for b in {words}; do for c in {words}; do done; done; done"
    );
    let (_, out) = run(&mut Session::default(), &line);
    assert_eq!(out, ["shell: stopped after 100000 commands on one line"]);
    // An empty while condition reads the last status, which a `true` keeps
    // at 0: uncharged, this is 10^8 rounds for 10^4 commands.
    let (_, out) = run(&mut Session::default(), "while; do while; do done; true; done");
    assert!(out.iter().any(|l| l == "shell: stopped after 100000 commands on one line"), "{out:?}");
    assert!(out.len() <= 32, "{out:?}");
}

#[test]
fn rows_through_a_pipe_are_held_to_the_steps() {
    // Each pipeline makes 100000 rows for one line of output; 4096 of them
    // is ~18 s of host CPU for one peer line unless the rows are charged.
    let words = vec!["w"; 64].join(" ");
    let line =
        format!("for a in {words}; do for b in {words}; do seq 100000 | tail -n 1; done; done");
    let (_, out) = run(&mut Session::default(), &line);
    assert_eq!(
        out.last().map(String::as_str),
        Some("shell: stopped after 100000 commands on one line")
    );
    assert!(out.len() < 64, "{} pipelines ran", out.len());
}
