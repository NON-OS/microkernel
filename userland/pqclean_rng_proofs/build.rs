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

// Compile, for the host, the C the kernel links for ML-DSA-65 and
// ML-KEM-768, with the kernel's own randombytes.c and its header first in
// the include order, as the kernel's build.rs has them.

use std::path::PathBuf;

fn c_files(dir: &PathBuf) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "c"))
        .collect();
    v.sort();
    v
}

fn build(name: &str, alg: &PathBuf, define: &str) {
    let root = PathBuf::from("../..");
    let support = root.join("src/crypto/pqclean_support");
    let common = root.join("third_party/pqclean/common");
    let mut b = cc::Build::new();
    b.files(c_files(alg));
    // One copy of SHAKE and the randomness hook, in the archive linked
    // last, so the other one's calls into them resolve.
    if name == "pq_mldsa" {
        b.file(common.join("fips202.c")).file(support.join("randombytes.c"));
    }
    b.include(&support).include(alg).include(&common).define(define, None).warnings(false);
    b.compile(name);
}

fn main() {
    let pq = PathBuf::from("../../third_party/pqclean");
    build("pq_mlkem", &pq.join("crypto_kem/ml-kem-768/clean"), "MLKEM768");
    build("pq_mldsa", &pq.join("crypto_sign/ml-dsa-65/clean"), "MLDSA65");
    println!("cargo:rerun-if-changed=../../src/crypto/pqclean_support");
    println!("cargo:rerun-if-changed=../../third_party/pqclean");
}
