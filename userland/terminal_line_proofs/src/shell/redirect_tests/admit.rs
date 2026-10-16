// NONOS Operating System (AGPL-3.0-or-later)
//! What a tool may be given: files for linux, none for a tool that never ends its input.

use crate::parse::parse;
use crate::redirect::plan;
use crate::tool_admit::admit;

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
