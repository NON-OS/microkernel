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

// Compile the pinned ML-DSA-65 C, and the SHAKE it needs, for whatever the
// crate is built for. A capsule triple is a JSON target clang does not
// know, so for it the C is built freestanding for the bare architecture,
// as nonos_qjs does. The allocator, exit and randomness calls the C makes
// are renamed and answered by this crate's Rust side.

use std::env;
use std::path::PathBuf;

fn main() {
    let root =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../third_party/pqclean");
    let src = root.join("crypto_sign/ml-dsa-65/clean");
    let common = root.join("common");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&src)
        .unwrap_or_else(|e| panic!("ML-DSA-65 source missing at {}: {e}", src.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "c"))
        .collect();
    files.sort();
    files.push(common.join("fips202.c"));

    let mut b = cc::Build::new();
    b.files(&files).include(&src).include(&common).define("MLDSA65", None).opt_level(2);
    // Every call the C makes outside itself is renamed to this crate's, so
    // the crate links beside any libc a capsule or a host already has.
    for name in ["malloc", "free", "exit"] {
        b.define(name, Some(format!("nonos_mldsa65_{name}").as_str()));
    }
    // The shared header already renames randombytes to this name.
    b.define("PQCLEAN_randombytes", Some("nonos_mldsa65_randombytes"));
    let names = std::fs::read_to_string("shake_names.txt").expect("shake_names.txt");
    for name in names.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
        b.define(name, Some(format!("nonos_mldsa65_{name}").as_str()));
    }
    b.warnings(false);
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("nonos") {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        b.compiler("clang").no_default_flags(true);
        b.flag(format!("--target={arch}-unknown-none"));
        b.flag("-ffreestanding").flag("-fno-builtin").flag("-fno-stack-protector");
        b.flag("-fPIC").flag("-O2").flag("-fwrapv");
        if arch == "x86_64" {
            b.flag("-mno-red-zone");
        }
        b.include(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("include"));
    }
    b.compile("nonos_mldsa65");

    println!("cargo:rerun-if-changed={}", src.display());
    println!("cargo:rerun-if-changed={}", common.join("fips202.c").display());
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=shake_names.txt");
}
