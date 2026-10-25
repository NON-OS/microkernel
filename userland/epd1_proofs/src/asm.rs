// NONOS Operating System (AGPL-3.0-or-later)
//! Reading assembly sources as instruction lists.

/// The instruction or directive on each line, comments and blanks gone.
pub fn code<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    let strip = |l: &str| l.split("/*").next().unwrap_or("").trim().to_string();
    let skip = |l: &&str| l.trim_start().starts_with('*') || l.trim_start().starts_with("/*");
    lines.into_iter().filter(|l| !skip(l)).map(strip).filter(|l| !l.is_empty()).collect()
}

/// The mnemonic, without a numeric label in front of it.
pub fn op(line: &str) -> &str {
    let line = match line.split_once(':') {
        Some((label, rest)) if label.chars().all(|c| c.is_ascii_digit()) => rest.trim(),
        _ => line,
    };
    line.split_whitespace().next().unwrap_or("")
}

pub fn mem(line: &str) -> bool {
    ["ldr", "str", "ldp", "stp"].contains(&op(line))
}

/// Branches whose target a predictor supplies.
pub fn indirect(line: &str) -> bool {
    ["br", "blr", "ret", "braa", "brab", "blraa", "blrab", "retaa", "retab"].contains(&op(line))
}

/// The body of `.macro name ...` up to its `.endm`, as code.
pub fn macro_body(text: &str, name: &str) -> Vec<String> {
    let head = format!(".macro {name}");
    let mut lines = text.lines().skip_while(|l| !l.trim().starts_with(&head));
    assert!(lines.next().is_some(), "{head} not found");
    code(lines.take_while(|l| l.trim() != ".endm"))
}
