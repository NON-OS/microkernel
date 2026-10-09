// NONOS Operating System (AGPL-3.0-or-later)
//! The line's operators, typed against a word or apart from it. `linux cat
//! </dev/null` handed cat the file name "</dev/null", and `echo hi>f` echoed
//! "hi>f": `<`, `>`, `>>` and `|` were operators only standing alone.

use crate::parse::parse;
use crate::redirect::{plan, Sink, Source};
use crate::statements::split_program;
use crate::tool_admit::admit;

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

fn planned(line: &str) -> (Vec<String>, String, String) {
    let argv = parse(line.as_bytes());
    let p = match plan(&argv.argv[..argv.argc]) {
        Ok(p) => p,
        Err(e) => panic!("{line}: {}", String::from_utf8_lossy(e)),
    };
    let w = p.words.iter().map(|w| String::from_utf8(w.to_vec()).unwrap()).collect();
    let src = match p.input {
        Source::Terminal => "tty".to_string(),
        Source::Null => "null".to_string(),
        Source::File(f) => format!("<{}", String::from_utf8_lossy(f)),
    };
    let sink = match p.output {
        Sink::Screen => "screen".to_string(),
        Sink::Null => "null".to_string(),
        Sink::File { path, append } => {
            format!("{}{}", if append { ">>" } else { ">" }, String::from_utf8_lossy(path))
        }
    };
    (w, src, sink)
}

fn refused(line: &str) -> String {
    let argv = parse(line.as_bytes());
    match plan(&argv.argv[..argv.argc]) {
        Ok(_) => panic!("{line}: planned"),
        Err(e) => String::from_utf8(e.to_vec()).unwrap(),
    }
}

#[test]
fn the_plan_takes_the_operators_out_of_the_words() {
    let (w, s, o) = planned("linux cat </dev/null");
    assert_eq!((w, s.as_str(), o.as_str()), (vec!["linux".into(), "cat".into()], "null", "screen"));
    let (w, s, o) = planned("linux cat < /some/file");
    assert_eq!(w, ["linux", "cat"]);
    assert_eq!((s.as_str(), o.as_str()), ("</some/file", "screen"));
    let (w, s, o) = planned("linux ls >out.txt");
    assert_eq!(w, ["linux", "ls"]);
    assert_eq!((s.as_str(), o.as_str()), ("tty", ">out.txt"));
    let (w, _, o) = planned("echo hi>>log");
    assert_eq!((w, o.as_str()), (vec!["echo".into(), "hi".into()], ">>log"));
    let (_, _, o) = planned("ls >/dev/null");
    assert_eq!(o, "null");
    // Words after the file are still the command's, as a POSIX shell reads them.
    let (w, _, o) = planned("echo a > f b");
    assert_eq!((w, o.as_str()), (vec!["echo".into(), "a".into(), "b".into()], ">f"));
    let (w, s, o) = planned("sort < in | uniq > out");
    assert_eq!(w, ["sort", "|", "uniq"]);
    assert_eq!((s.as_str(), o.as_str()), ("<in", ">out"));
    let (w, s, o) = planned("echo \"a>b\"");
    assert_eq!((w, s.as_str(), o.as_str()), (vec!["echo".into(), "a>b".into()], "tty", "screen"));
}

#[test]
fn a_redirect_the_terminal_cannot_honour_is_refused() {
    assert_eq!(refused("cat <"), "redirect: expected a file path after <");
    assert_eq!(refused("ls >"), "redirect: expected a file path after >");
    assert_eq!(refused("ls > | grep x"), "redirect: expected a file path after >");
    assert_eq!(refused("cat < a < b"), "redirect: one < per command");
    assert_eq!(refused("ls > a >> b"), "redirect: one > or >> per command");
    assert_eq!(refused("ls > f | grep x"), "redirect: > goes in the last stage of a pipe");
    assert_eq!(refused("ls | grep x < f"), "redirect: < goes in the first stage of a pipe");
    for line in ["ls 2>f", "ls 2>&1", "ls >&2", "ls 2>>f"] {
        assert_eq!(
            refused(line),
            "redirect: a numbered stream (2>, 2>&1) is not taken; only <, > and >> are",
            "{line}"
        );
    }
}

fn admitted(line: &str) -> Result<(), String> {
    let argv = parse(line.as_bytes());
    let p = plan(&argv.argv[..argv.argc]).unwrap();
    admit(p.words[0], &p).map_err(|e| String::from_utf8(e).unwrap())
}

#[test]
fn linux_takes_a_file_in_and_out_and_a_pipe_is_said_to_be_inside_it() {
    for line in ["linux cat </dev/null", "linux cat < /some/file", "linux ls >out.txt"] {
        assert_eq!(admitted(line), Ok(()), "{line}");
    }
    assert_eq!(admitted("linux ls >>log"), Ok(()));
    assert_eq!(admitted("linux ls >/dev/null"), Ok(()));
    assert_eq!(
        admitted("linux ls | grep x").unwrap_err(),
        "linux: a pipe into or out of a Linux program is done inside it; use: linux sh -c 'prog | grep x'"
    );
}

#[test]
fn a_tool_with_no_end_of_input_is_refused_a_file_before_it_starts() {
    assert_eq!(
        admitted("jsonxf < data.json").unwrap_err(),
        "jsonxf: < is not taken: this program has no end of input to be given after a file"
    );
    assert!(admitted("jsonxf </dev/null").is_err());
    assert_eq!(admitted("tokei > counts.txt"), Ok(()));
    assert_eq!(admitted("grex abc"), Ok(()));
    assert!(admitted("grex abc | sort").unwrap_err().starts_with("grex: "));
}
