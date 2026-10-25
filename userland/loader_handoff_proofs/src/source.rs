// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's own source, read for the order it keeps.

pub(crate) fn raw(path: &str) -> String {
    std::fs::read_to_string(format!("../../{path}")).unwrap()
}

pub(crate) fn code(path: &str) -> String {
    let src = std::fs::read_to_string(format!("../../src/{path}")).unwrap();
    src.lines().map(|l| l.split("//").next().unwrap_or("").trim()).collect::<Vec<_>>().join("\n")
}

pub(crate) fn order(src: &str, marks: &[&str]) {
    let mut from = 0;
    for mark in marks {
        let at = src[from..].find(mark).unwrap_or_else(|| panic!("no `{mark}` after {from}"));
        from += at + mark.len();
    }
}
