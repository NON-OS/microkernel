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

//! The fixture: real enrollments by `nonos-stark-enroll` with real STARK
//! proofs (tests/fixture/README.md), and the committed release material.

#![allow(dead_code)]

use std::path::PathBuf;
use std::process::Command;

pub fn fx(name: &str) -> String {
    format!("{}/tests/fixture/{name}", env!("CARGO_MANIFEST_DIR"))
}

pub fn top(name: &str) -> String {
    format!("{}/../{name}", env!("CARGO_MANIFEST_DIR"))
}

/// The committed trust material, named whole in one literal so the source of
/// this crate's test lane carries it (tools/nix/inputs.py follows it).
pub fn trust(name: &str) -> String {
    format!("{}/../nonos-data/trust/{name}", env!("CARGO_MANIFEST_DIR"))
}

pub fn bytes(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

pub fn root(path: &str) -> [u8; 32] {
    bytes(path).try_into().expect("a 32-byte root")
}

/// A scratch directory of the test's own, removed and made afresh.
pub fn scratch(name: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

pub fn write(dir: &std::path::Path, name: &str, b: &[u8]) -> String {
    let p = dir.join(name);
    std::fs::write(&p, b).unwrap();
    p.to_str().unwrap().to_string()
}

/// The shipping binary, run: its exit code and standard output.
pub fn cli(args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_nonos-proof-verify")).args(args).output().unwrap();
    (out.status.code().expect("an exit code"), String::from_utf8(out.stdout).unwrap())
}

pub fn json(args: &[&str]) -> (i32, serde_json::Value) {
    let mut a = args.to_vec();
    a.push("--json");
    let (code, out) = cli(&a);
    let v: serde_json::Value =
        serde_json::from_str(&out).unwrap_or_else(|e| panic!("not JSON ({e}): {out}"));
    assert_eq!(v["exit_code"], code, "the document's exit code is the process's");
    (code, v)
}

/// A tiny deterministic generator, so a failing case is the same every run.
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}
