// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Every install fetch that goes over the route takes, in a development
//! build, a mirror on the local network directly, as docs/build/dev-mirror.md
//! says: a call of route.rs's `exchange` in capsule_linux has an
//! `is_local` branch just before it, or is the release build's own line
//! under `not(feature = "dev-mirror")`. A pinned content tar (a source
//! recipe's, an in-tree tool's) asked over the route alone waited for
//! Anyone with the host's relay a direct dial away.

use std::fs;
use std::path::{Path, PathBuf};

const SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_linux/src");
const RELEASE_LINE: &str = "#[cfg(not(feature = \"dev-mirror\"))]";
/// How far above a call its branch may be.
const NEAR: usize = 6;

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).expect("capsule_linux's sources").flatten() {
        let p = e.path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Each `file:line` calling exchange( with no local branch near it.
fn unsplit(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    sources(root, &mut files);
    let mut out = Vec::new();
    for f in files {
        let text = fs::read_to_string(&f).expect("a source file");
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            let call = code.contains("exchange(") && !code.contains("compare_exchange(");
            if !call || code.contains("fn exchange(") || code.trim_start().starts_with("use ") {
                continue;
            }
            let near = &lines[i.saturating_sub(NEAR)..i];
            if !near.iter().any(|l| l.contains("is_local(") || l.trim() == RELEASE_LINE) {
                out.push(format!("{}:{}", f.display(), i + 1));
            }
        }
    }
    out
}

#[test]
fn every_fetch_over_the_route_takes_a_local_mirror_directly_in_a_development_build() {
    assert_eq!(unsplit(Path::new(SRC)), Vec::<String>::new());
}

#[test]
fn a_fetch_over_the_route_alone_is_found() {
    let dir = std::env::temp_dir().join(format!("mirror-paths-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let body = "fn ask(at: &str) -> Option<Vec<u8>> {\n    let raw = exchange(at, 80, b\"GET\", 1, &complete);\n    raw\n}\n";
    fs::write(dir.join("route_only.rs"), body).unwrap();
    let found = unsplit(&dir);
    fs::remove_dir_all(&dir).unwrap();
    assert_eq!(found.len(), 1, "{found:?}");
}
