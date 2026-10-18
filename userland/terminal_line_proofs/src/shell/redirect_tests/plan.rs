// NONOS Operating System (AGPL-3.0-or-later)
//! The plan takes the operators out of the words and names the input and output.

use crate::parse::parse;
use crate::redirect::{plan, Sink, Source};

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
