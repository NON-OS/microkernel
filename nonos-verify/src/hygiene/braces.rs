// Brace depth through Rust source, line by line, with braces inside strings,
// raw strings, character literals and comments left out, so an inline test
// module is skipped to its own closing brace and no further.

#[derive(Clone, Copy, PartialEq)]
enum In {
    Code,
    Str,
    Raw(usize),
    Block(usize),
}

pub struct Braces {
    state: In,
}

impl Braces {
    pub fn new() -> Self {
        Braces { state: In::Code }
    }

    /// The change in depth across `line`.
    pub fn step(&mut self, line: &str) -> i64 {
        let b = line.as_bytes();
        let mut depth = 0i64;
        let mut i = 0;
        while i < b.len() {
            match self.state {
                In::Str => match b[i] {
                    b'\\' => i += 1,
                    b'"' => self.state = In::Code,
                    _ => {}
                },
                In::Raw(hashes) => {
                    if b[i] == b'"'
                        && b[i + 1..].iter().take(hashes).filter(|&&c| c == b'#').count() == hashes
                    {
                        self.state = In::Code;
                        i += hashes;
                    }
                }
                In::Block(n) => {
                    if b[i..].starts_with(b"*/") {
                        self.state = if n == 1 { In::Code } else { In::Block(n - 1) };
                        i += 1;
                    } else if b[i..].starts_with(b"/*") {
                        self.state = In::Block(n + 1);
                        i += 1;
                    }
                }
                In::Code => {
                    if b[i..].starts_with(b"//") {
                        break;
                    } else if b[i..].starts_with(b"/*") {
                        self.state = In::Block(1);
                        i += 1;
                    } else if let Some((hashes, len)) = raw_open(b, i) {
                        self.state = In::Raw(hashes);
                        i += len - 1;
                    } else if b[i] == b'"' {
                        self.state = In::Str;
                    } else if b[i] == b'\'' {
                        i += char_literal(&b[i..]);
                    } else if b[i] == b'{' {
                        depth += 1;
                    } else if b[i] == b'}' {
                        depth -= 1;
                    }
                }
            }
            i += 1;
        }
        depth
    }
}

/// When a raw string (`r"`, `r#"`, `br##"`) opens at `at`: its number of
/// `#`s and the length of its opening, quote included.
fn raw_open(b: &[u8], at: usize) -> Option<(usize, usize)> {
    if at > 0 && (b[at - 1].is_ascii_alphanumeric() || b[at - 1] == b'_') {
        return None;
    }
    let rest = b[at..].strip_prefix(b"br").or_else(|| b[at..].strip_prefix(b"r"))?;
    let hashes = rest.iter().take_while(|&&c| c == b'#').count();
    let prefix = b.len() - at - rest.len();
    (rest.get(hashes) == Some(&b'"')).then_some((hashes, prefix + hashes + 1))
}

/// How far past the opening quote a character literal ends, or 0 for a
/// lifetime, whose quote opens nothing.
fn char_literal(b: &[u8]) -> usize {
    match b.get(1) {
        Some(b'\\') => {
            b.get(3..).and_then(|r| r.iter().position(|&c| c == b'\'')).map_or(0, |p| p + 3)
        }
        Some(_) if b.get(2) == Some(&b'\'') => 2,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::Braces;

    fn depth(lines: &[&str]) -> Vec<i64> {
        let mut b = Braces::new();
        lines.iter().map(|l| b.step(l)).collect()
    }

    #[test]
    fn braces_in_code_count() {
        assert_eq!(depth(&["mod t {", "    fn f() { x }", "}"]), [1, 0, -1]);
    }

    #[test]
    fn braces_in_strings_chars_and_comments_do_not() {
        assert_eq!(
            depth(&[r#"let s = "{{";"#, "let c = '{';", "// }", "/* { */", "let e = '\\'';"]),
            [0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn a_raw_string_may_hold_quotes_and_span_lines() {
        assert_eq!(depth(&["let j = r#\"{\"a\":", "\"b\"}\"#; {"]), [0, 1]);
    }

    #[test]
    fn a_lifetime_is_not_a_character_literal() {
        assert_eq!(depth(&["fn f<'a>(x: &'a str) {"]), [1]);
    }
}
