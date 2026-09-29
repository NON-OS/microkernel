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

//! Pins the package keyrings into the capsule. NONOS_PACMAN_KEYRING and
//! NONOS_DEB_KEYRING each name a keyring file. Unset, pacman's is empty and
//! every pacman install is refused; Debian's is Kali's pinned archive key. A named file that cannot
//! be read fails the build, since an image that silently lost its keyring
//! would look like one built without it.

use std::path::{Path, PathBuf};
use std::{env, fs, process};

const SETTINGS: [&str; 9] = [
    "NONOS_PACMAN_MIRROR",
    "NONOS_PACMAN_HOST",
    "NONOS_PACMAN_PATH",
    "NONOS_PACMAN_REPOS",
    "NONOS_DEB_MIRROR",
    "NONOS_DEB_HOST",
    "NONOS_DEB_ROOT",
    "NONOS_DEB_SUITE",
    "NONOS_DEB_COMPONENTS",
];

fn main() {
    for var in SETTINGS {
        println!("cargo:rerun-if-env-changed={var}");
    }
    let Some(out) = env::var_os("OUT_DIR").map(PathBuf::from) else { fail("no OUT_DIR") };
    pin("NONOS_PACMAN_KEYRING", &out.join("pacman-keyring.gpg"), None);
    // Kali's archive key is pinned by default; see design/package-trust.md.
    pin("NONOS_DEB_KEYRING", &out.join("deb-keyring.gpg"), Some("keys/kali/archive-key-2025.asc"));
}

/// The keyring `var` names, else `default` (relative to this crate), else none.
fn pin(var: &str, to: &Path, default: Option<&str>) {
    println!("cargo:rerun-if-env-changed={var}");
    let path = env::var(var).ok().or_else(|| default.map(String::from));
    let ring = match path {
        Some(path) => {
            println!("cargo:rerun-if-changed={path}");
            fs::read(&path).unwrap_or_else(|e| fail(&format!("{var}={path}: {e}")))
        }
        None => Vec::new(),
    };
    if let Err(e) = fs::write(to, ring) {
        fail(&format!("writing {}: {e}", to.display()));
    }
}

fn fail(why: &str) -> ! {
    eprintln!("{why}");
    process::exit(1)
}
