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

use super::scan::{arches, declared, defined};
use std::path::{Path, PathBuf};

/// Each declaration's missing (arch, name), over the tree at `root`.
fn undefined(root: &Path) -> Vec<String> {
    let src = root.join("src");
    let mut out = Vec::new();
    for (file, name) in declared(&src) {
        for arch in arches(&src, &file) {
            if !defined(root, arch, &name) {
                out.push(format!("{name} ({arch}, {})", file.display()));
            }
        }
    }
    out
}

#[test]
fn every_link_symbol_the_kernel_declares_is_defined() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let found = declared(&root.join("src")).len();
    assert!(found >= 25, "found only {found} declarations");
    assert_eq!(undefined(&root), Vec::<String>::new());
}

/// A tree with the three scripts, each defining `__a`, and one source file.
fn tree(tag: &str, path: &str, decl: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("link_symbols_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for s in ["linker.ld", "linker_aarch64.ld", "linker_riscv64.ld"] {
        std::fs::create_dir_all(&root).expect("tree");
        std::fs::write(root.join(s), "    __a = .;\n").expect("script");
    }
    let file = root.join("src").join(path);
    std::fs::create_dir_all(file.parent().expect("dir")).expect("dir");
    std::fs::write(file, format!("extern \"C\" {{\n    static {decl}: u8;\n}}\n")).expect("src");
    root
}

#[test]
fn a_declaration_no_script_defines_is_caught() {
    let root = tree("missing", "memory/state.rs", "__percpu_start");
    assert_eq!(undefined(&root).len(), 3, "{:?}", undefined(&root));
    let root = tree("defined", "memory/state.rs", "__a");
    assert!(undefined(&root).is_empty());
}

#[test]
fn an_arch_file_needs_only_its_own_script_or_its_own_label() {
    let root = tree("x86", "arch/x86_64/boot.rs", "__b");
    assert_eq!(undefined(&root).len(), 1, "{:?}", undefined(&root));
    std::fs::write(root.join("linker.ld"), "__b = .;\n").expect("script");
    assert!(undefined(&root).is_empty());
    let root = tree("label", "arch/aarch64/vec.rs", "__v");
    std::fs::write(root.join("src/arch/aarch64/vec.S"), ".global __v\n__v:\n").expect("asm");
    assert!(undefined(&root).is_empty());
}
