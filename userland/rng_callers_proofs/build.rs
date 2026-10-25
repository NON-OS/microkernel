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

//! The kernel's stack canary keeps its fields private and gives no way to
//! read its per-CPU values or its rotation count, so its proofs must sit
//! inside its module. Its file opens with a module doc comment, which a
//! module body made by `include!` may not hold; this copies the file to
//! the build directory with only those lines left out, every line of code
//! as it is, and again whenever the kernel's file changes.

use std::{env, fs, path::Path};

const CANARY: &str = "../../src/security/policy/advanced/canary.rs";

fn main() {
    println!("cargo:rerun-if-changed={CANARY}");
    let Ok(source) = fs::read_to_string(CANARY) else {
        panic!("the kernel's canary.rs is not at {CANARY}");
    };
    let body: String = source
        .lines()
        .filter(|l| !l.trim_start().starts_with("//!"))
        .flat_map(|l| [l, "\n"])
        .collect();
    let Some(out) = env::var_os("OUT_DIR") else {
        panic!("cargo sets OUT_DIR for a build script");
    };
    if fs::write(Path::new(&out).join("advanced_canary.rs"), body).is_err() {
        panic!("the build directory is not writable");
    }
}
