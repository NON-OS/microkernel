// NONOS Operating System (AGPL-3.0-or-later)
//! The loader's own source, read for what it says and the order it keeps.

/// A loader file with its comments taken out: `/* */` blocks and `//` tails.
pub(crate) fn code(path: &str) -> String {
    let src = std::fs::read_to_string(format!("../../nonos-bootloader/src/{path}")).unwrap();
    let mut out = String::new();
    let mut rest = src.as_str();
    while let Some(at) = rest.find("/*") {
        out.push_str(&rest[..at]);
        rest = rest[at..].split_once("*/").map_or("", |(_, after)| after);
    }
    out.push_str(rest);
    out.lines().map(|l| l.split("//").next().unwrap_or("").trim()).collect::<Vec<_>>().join("\n")
}

/// Each mark found after the one before it, or a panic naming the first
/// that is not.
pub(crate) fn order(src: &str, marks: &[&str]) {
    let mut from = 0;
    for mark in marks {
        let at = src[from..].find(mark).unwrap_or_else(|| panic!("no `{mark}` after {from}"));
        from += at + mark.len();
    }
}
