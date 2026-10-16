// NONOS Operating System (AGPL-3.0-or-later)
//! `<`, `>`, `>>` and `|` are operators against a word, apart from it, and not in quotes.

use crate::parse::parse;
use crate::statements::split_program;

fn words(line: &str) -> Vec<String> {
    let argv = parse(line.as_bytes());
    argv.argv[..argv.argc].iter().map(|w| String::from_utf8(w.to_vec()).unwrap()).collect()
}

#[test]
fn an_operator_against_a_word_is_still_an_operator() {
    assert_eq!(words("linux cat </dev/null"), ["linux", "cat", "<", "/dev/null"]);
    assert_eq!(words("echo hi>f"), ["echo", "hi", ">", "f"]);
    assert_eq!(words("echo hi >f"), ["echo", "hi", ">", "f"]);
    assert_eq!(words("echo hi>> f"), ["echo", "hi", ">>", "f"]);
    assert_eq!(words("a>b"), ["a", ">", "b"]);
    assert_eq!(words("ls|grep x"), ["ls", "|", "grep", "x"]);
    assert_eq!(words("sort<in>out"), ["sort", "<", "in", ">", "out"]);
    assert_eq!(words("echo a>>b|c"), ["echo", "a", ">>", "b", "|", "c"]);
}

#[test]
fn spaced_operators_read_as_before() {
    assert_eq!(words("linux cat < /etc/motd"), ["linux", "cat", "<", "/etc/motd"]);
    assert_eq!(words("ls > out.txt"), ["ls", ">", "out.txt"]);
    assert_eq!(words("ls | grep a"), ["ls", "|", "grep", "a"]);
}

#[test]
fn quoted_operators_are_text() {
    assert_eq!(words("echo \"a>b\""), ["echo", "a>b"]);
    assert_eq!(words("echo 'x|y<z'"), ["echo", "x|y<z"]);
    assert_eq!(words("linux sh -c 'prog < file'"), ["linux", "sh", "-c", "prog < file"]);
}

#[test]
fn a_stream_number_stays_with_its_operator() {
    // `1>` and `0<` are the plain forms; any other number is one token the
    // redirect plan refuses, never a stray argument "2".
    assert_eq!(words("ls 1>f"), ["ls", ">", "f"]);
    assert_eq!(words("ls 1>>f"), ["ls", ">>", "f"]);
    assert_eq!(words("cat 0<f"), ["cat", "<", "f"]);
    assert_eq!(words("ls 2>f"), ["ls", "2>", "f"]);
    assert_eq!(words("ls 2>&1"), ["ls", "2>&1"]);
    assert_eq!(words("ls x2>f"), ["ls", "x2", ">", "f"]);
}

#[test]
fn an_ampersand_after_a_redirect_is_not_a_background_mark() {
    let stmts = split_program(b"ls 2>&1; echo x");
    assert_eq!(stmts.len(), 2);
    assert_eq!(stmts[0].body, b"ls 2>&1");
    assert!(!stmts[0].background);
    let stmts = split_program(b"ls >f &");
    assert_eq!(stmts.len(), 1);
    assert!(stmts[0].background);
}
