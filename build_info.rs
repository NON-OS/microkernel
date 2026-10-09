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

/* The build identity the kernel reads through env!. The release number is read
 * from the VERSION file, not restated here. */

use std::env;
use std::fs;
use std::process::Command;

/* The channel this tree is released on. */
const RELEASE_CHANNEL: &str = "beta";

pub(crate) fn embed_kernel_build_info() {
    let build_time = match std::env::var("SOURCE_DATE_EPOCH") {
        Ok(epoch) => format!("epoch:{}", epoch.trim()),
        Err(_) => "reproducible:none".to_string(),
    };
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    println!("cargo:rustc-env=NONOS_KERNEL_BUILD_TIME={}", build_time);

    if let Ok(output) =
        std::process::Command::new("git").args(["rev-parse", "--short", "HEAD"]).output()
    {
        let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("cargo:rustc-env=NONOS_KERNEL_GIT_COMMIT={}", commit);
    } else {
        println!("cargo:rustc-env=NONOS_KERNEL_GIT_COMMIT=unknown");
    }

    println!("cargo:rustc-env=NONOS_KERNEL_RUSTC_VERSION={}", rustc_version());
    println!("cargo:rustc-env=NONOS_KERNEL_NAME=NONOS Kernel");
    println!("cargo:rustc-env=NONOS_KERNEL_VERSION={}", release_version());
    println!("cargo:rustc-env=NONOS_RELEASE_CHANNEL={}", RELEASE_CHANNEL);
}

/* VERSION holds one bare version string. Anything else fails the build rather
 * than reach the kernel, and a line break would read as a second directive. */
fn release_version() -> String {
    println!("cargo:rerun-if-changed=VERSION");
    let text = fs::read_to_string("VERSION").expect("read VERSION");
    let version = text.trim();
    if version.is_empty() || version.contains(char::is_whitespace) {
        panic!("VERSION must hold a single version string, found {version:?}");
    }
    version.to_string()
}

/* What the compiler cargo resolved for this build says about itself. */
fn rustc_version() -> String {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let out = Command::new(rustc).arg("--version").output().ok().filter(|o| o.status.success());
    let line = out.map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    line.filter(|l| !l.is_empty() && !l.contains('\n')).unwrap_or_else(|| "unknown".to_string())
}
