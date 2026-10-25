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

//! The declarations, and where a symbol may be defined: the linker script
//! of each image the declaring file links into, or a label in that
//! architecture's own assembly.

use super::walk::files;
use std::path::{Path, PathBuf};

const SCRIPTS: [(&str, &str); 3] =
    [("x86_64", "linker.ld"), ("aarch64", "linker_aarch64.ld"), ("riscv64", "linker_riscv64.ld")];

/// Every (file, name) declared as a static inside an `extern "C"` block.
pub(super) fn declared(src: &Path) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for path in files(src, "rs") {
        let text = std::fs::read_to_string(&path).expect("readable");
        for block in text.split("extern \"C\" {").skip(1) {
            let body = &block[..block.find('}').unwrap_or(block.len())];
            for line in body.lines().map(str::trim) {
                let rest = line.strip_prefix("pub ").unwrap_or(line);
                if let Some(decl) = rest.strip_prefix("static ") {
                    let name = decl.trim_start_matches("mut ").split(':').next().unwrap_or("");
                    out.push((path.clone(), name.trim().to_string()));
                }
            }
        }
    }
    out
}

/// The architectures a file links into: its own, or all three.
pub(super) fn arches(src: &Path, file: &Path) -> Vec<&'static str> {
    let own = SCRIPTS.iter().find(|(a, _)| file.starts_with(src.join("arch").join(a)));
    own.map_or_else(|| SCRIPTS.iter().map(|s| s.0).collect(), |s| vec![s.0])
}

/// Whether `name` is defined for `arch`: assigned in its linker script,
/// or a label in its assembly under src/arch/<arch>.
pub(super) fn defined(root: &Path, arch: &str, name: &str) -> bool {
    let script = SCRIPTS.iter().find(|s| s.0 == arch).expect("known arch").1;
    let text = std::fs::read_to_string(root.join(script)).unwrap_or_default();
    let assigned = text.lines().any(|l| {
        let l = l.trim();
        l.strip_prefix(name).is_some_and(|r| r.trim_start().starts_with('='))
            || l.contains(&format!("PROVIDE({name}"))
    });
    let dir = root.join("src/arch").join(arch);
    assigned || ["S", "s", "rs"].iter().flat_map(|e| files(&dir, e)).any(|p| has_label(&p, name))
}

fn has_label(path: &Path, name: &str) -> bool {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let label = format!("{name}:");
    text.lines().any(|l| l.trim().trim_start_matches('"').starts_with(&label))
}
