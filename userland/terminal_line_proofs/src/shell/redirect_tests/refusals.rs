// NONOS Operating System (AGPL-3.0-or-later)
//! A redirect the terminal cannot honour is refused, with the reason.

use crate::parse::parse;
use crate::redirect::plan;

fn refused(line: &str) -> String {
    let argv = parse(line.as_bytes());
    match plan(&argv.argv[..argv.argc]) {
        Ok(_) => panic!("{line}: planned"),
        Err(e) => String::from_utf8(e.to_vec()).unwrap(),
    }
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
