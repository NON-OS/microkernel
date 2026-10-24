// NONOS Operating System (AGPL-3.0-or-later)
//! Every aarch64 file holding a per-CPU table: none folds an index into the
//! table (`% MAX_CPUS`, `.min(MAX_CPUS - 1)`), since folding is how two CPUs
//! come to share a slot. Indices are the roster's, below MAX_CPUS by
//! construction, or are looked up with `get` and refused past the end.

use std::fs;
use std::path::Path;

fn rust_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn code(src: &str) -> String {
    let lines = src.lines().map(|l| l.split("//").next().unwrap_or(""));
    lines.collect::<Vec<_>>().join("\n")
}

#[test]
fn no_aarch64_file_folds_a_cpu_index() {
    let mut files = Vec::new();
    rust_files(Path::new("../../src/arch/aarch64"), &mut files);
    assert!(files.len() > 300, "the sweep reached {} files", files.len());
    for file in files {
        let src = code(&fs::read_to_string(&file).unwrap());
        for fold in ["% MAX_CPUS", "%MAX_CPUS", "MAX_CPUS - 1"] {
            assert!(!src.contains(fold), "{} folds a CPU index: {fold}", file.display());
        }
        // The old guess: a CPU the roster does not hold taken for CPU 0.
        let guess = src.lines().any(|l| l.contains("index_of(") && l.contains("unwrap_or(0)"));
        assert!(!guess, "{} falls back to CPU 0", file.display());
    }
}

#[test]
fn the_two_table_sizes_are_one() {
    let roster = fs::read_to_string("../../src/arch/aarch64/boot/multicore/roster.rs").unwrap();
    assert!(roster.contains("const _: () = assert!(MAX_CPUS == crate::smp::MAX_CPUS);"));
}
