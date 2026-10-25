// NONOS Operating System (AGPL-3.0-or-later)
//! Which lines of a kernel file are x86_64-only, by each ledger's rule.

/// A line that gates on x86_64: `target_arch = "x86_64"`, however spaced, or
/// a gate written the other way round, `not(...)` of the other architectures.
pub fn names_x86_cfg(line: &str) -> bool {
    let tight: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    let others = ["target_arch=\"aarch64\"", "target_arch=\"riscv64\""];
    tight.contains("target_arch=\"x86_64\"")
        || tight.contains("not(") && others.iter().any(|o| tight.contains(o))
}

/// A line that reaches into the x86_64 tree by path: `crate::arch::x86_64`,
/// or `x86_64::` inside a `crate::arch::{...}` group on the same line.
pub fn names_x86_path(line: &str) -> bool {
    const PATH: &str = "crate::arch::x86_64";
    let direct = line.match_indices(PATH).any(|(at, _)| {
        let rest = &line[at + PATH.len()..];
        !rest.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
    });
    direct || line.contains("crate::arch::{") && line.contains("x86_64::")
}

/// What a ledger counts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rule {
    Cfg,
    Path,
}

/// The lines of `text` the rule takes, and, for `Path`, each line of a
/// `crate::arch::{` group spread over lines that names `x86_64::`.
pub fn hits(text: &str, rule: Rule) -> usize {
    let (mut group, mut n) = (false, 0);
    for line in text.lines() {
        let one = match rule {
            Rule::Cfg => names_x86_cfg(line),
            Rule::Path => names_x86_path(line) || group && line.contains("x86_64::"),
        };
        n += usize::from(one);
        group = (group || line.contains("crate::arch::{")) && !line.contains('}');
    }
    n
}
