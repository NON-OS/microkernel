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

//! The vendor table (src/vendor/table.rs), built from
//! userland/capsule_linux/policies.txt with this crate's own code: the
//! modules the encoding needs are the shipping files, mounted by path.

use std::path::Path;
use std::{env, process};

#[path = "src/bucket.rs"]
pub mod bucket;
#[path = "src/encode.rs"]
pub mod encode;
#[path = "src/error.rs"]
pub mod error;
#[path = "src/layout.rs"]
pub mod layout;
#[path = "src/mask.rs"]
pub mod mask;
#[path = "src/policy.rs"]
pub mod policy;
#[path = "src/syscall_set.rs"]
pub mod syscall_set;
#[path = "src/unlisted.rs"]
pub mod unlisted;

#[path = "src/dests/mod.rs"]
pub mod dests;
#[path = "build/mod.rs"]
mod tools;

use tools::{guard, table};

const POLICIES: &str = "../capsule_linux/policies.txt";

fn main() {
    println!("cargo:rerun-if-changed=build");
    println!("cargo:rerun-if-changed=src");
    for var in ["NONOS_LINUX_USERLAND", "NONOS_LINUX_GUEST_POLICIES", "NONOS_IN_FLAKE"] {
        println!("cargo:rerun-if-env-changed={var}");
    }
    // Without the vendor table there is nothing to build, and nothing to guard.
    if env::var_os("CARGO_FEATURE_VENDOR").is_none() {
        return;
    }
    let out =
        Path::new(&env::var_os("OUT_DIR").expect("cargo sets OUT_DIR")).join("vendor_table.rs");
    let rows = table::write(Path::new(POLICIES), &out).unwrap_or_else(|why| fail(&why));
    let [target, host, profile] =
        ["TARGET", "HOST", "PROFILE"].map(|v| env::var(v).unwrap_or_default());
    let in_flake = env::var("NONOS_IN_FLAKE").is_ok_and(|v| v == "1");
    let build = guard::Build { target: &target, host: &host, profile: &profile, in_flake };
    let userland = env::var_os("NONOS_LINUX_USERLAND").is_some();
    guard::may_build(&build, userland, rows).unwrap_or_else(|why| fail(&why));
}

fn fail(why: &str) -> ! {
    eprintln!("nonos_linux_policy: {why}");
    process::exit(1)
}
