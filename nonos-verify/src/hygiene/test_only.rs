// Which Rust source compiles only for tests: every file a `#[cfg(test)] mod`
// declaration mounts, with the modules under it, and the body of an inline
// `#[cfg(test)] mod name { ... }`. Test code may unwrap and panic; a failing
// assertion is the point, and the hygiene check is about what ships.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::braces::Braces;
use super::roots::{is_rust, skip};

/// One `mod` declaration: where it sits, whether `#[cfg(test)]` gates it,
/// the `#[path]` it names, and whether its body is inline.
struct Decl {
    first: usize,
    line: usize,
    name: String,
    path: Option<String>,
    test: bool,
    inline: bool,
}

fn decls(text: &str) -> Vec<Decl> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let first = i;
        let (mut test, mut path) = (false, None);
        while i < lines.len() && lines[i].trim_start().starts_with("#[") {
            let t = lines[i].trim();
            test |= t == "#[cfg(test)]";
            if let Some(p) = t.strip_prefix("#[path = \"").and_then(|r| r.strip_suffix("\"]")) {
                path = Some(p.to_string());
            }
            i += 1;
        }
        if let Some(d) = lines.get(i).and_then(|l| declared(l)) {
            let (name, inline) = d;
            out.push(Decl { first, line: i, name, path, test, inline });
        }
        i += 1;
    }
    out
}

/// The module a line declares, and whether its body follows inline.
fn declared(line: &str) -> Option<(String, bool)> {
    let t = line.trim();
    let t =
        ["pub(crate) ", "pub(super) ", "pub "].iter().find_map(|v| t.strip_prefix(v)).unwrap_or(t);
    let rest = t.strip_prefix("mod ")?;
    let name: String =
        rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    let tail = rest[name.len()..].trim();
    if name.is_empty() || !(tail == ";" || tail.starts_with('{')) {
        return None;
    }
    Some((name, tail.starts_with('{')))
}

/// The files a declaration in `file` may mount.
fn mounted(file: &Path, d: &Decl) -> Vec<PathBuf> {
    let dir = file.parent().unwrap_or(Path::new("."));
    if let Some(p) = &d.path {
        return vec![dir.join(p)];
    }
    let owner = match file.file_name().and_then(|n| n.to_str()) {
        Some("mod.rs" | "lib.rs" | "main.rs") => dir.to_path_buf(),
        _ => dir.join(file.file_stem().unwrap_or_default()),
    };
    vec![owner.join(format!("{}.rs", d.name)), owner.join(&d.name).join("mod.rs")]
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if is_rust(&path) && !skip(&path) {
            out.push(path);
        }
    }
}

/// Every file under `roots` that only a test build compiles.
pub fn files(roots: &[&str]) -> HashSet<PathBuf> {
    let mut all = Vec::new();
    for root in roots {
        rust_files(Path::new(root), &mut all);
    }
    let mut tests = HashSet::new();
    let mut todo = Vec::new();
    for file in &all {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        for d in decls(&text).iter().filter(|d| d.test && !d.inline) {
            todo.extend(mounted(file, d));
        }
    }
    // A module under a test-only file is test-only, gated or not.
    while let Some(file) = todo.pop() {
        if !file.is_file() || !tests.insert(file.clone()) {
            continue;
        }
        let text = std::fs::read_to_string(&file).unwrap_or_default();
        for d in decls(&text).iter().filter(|d| !d.inline) {
            todo.extend(mounted(&file, d));
        }
    }
    tests
}

/// For each line of `text`, whether it lies in an inline `#[cfg(test)]` module.
pub fn lines(text: &str) -> Vec<bool> {
    let all: Vec<&str> = text.lines().collect();
    let mut mask = vec![false; all.len()];
    for d in decls(text).iter().filter(|d| d.test && d.inline) {
        let mut braces = Braces::new();
        let mut depth = 0;
        let mut end = d.line;
        for (n, line) in all.iter().enumerate().skip(d.line) {
            depth += braces.step(line);
            end = n;
            if depth <= 0 {
                break;
            }
        }
        mask[d.first..=end].iter_mut().for_each(|m| *m = true);
    }
    mask
}

#[cfg(test)]
mod tests {
    use super::{decls, lines};

    #[test]
    fn an_inline_test_module_is_masked_to_its_closing_brace() {
        let text =
            "fn ship() {}\n#[cfg(test)]\nmod t {\n    fn a() { x.unwrap(); }\n}\nfn after() {}\n";
        assert_eq!(lines(text), [false, true, true, true, true, false]);
    }

    #[test]
    fn a_gated_file_declaration_names_its_path() {
        let d = decls("#[cfg(test)]\n#[path = \"nonce_test.rs\"]\nmod nonce_test;\nmod shipped;\n");
        assert_eq!(d.len(), 2);
        assert!(d[0].test && !d[0].inline && d[0].path.as_deref() == Some("nonce_test.rs"));
        assert!(!d[1].test && d[1].name == "shipped");
    }

    #[test]
    fn an_ungated_inline_module_is_scanned() {
        assert_eq!(lines("mod live {\n    x.unwrap();\n}\n"), [false, false, false]);
    }
}
