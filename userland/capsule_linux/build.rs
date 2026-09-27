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

//! Pins the pacman keyring into the capsule. NONOS_PACMAN_KEYRING names a
//! `gpg --export` file; unset, the keyring is empty and every pacman install
//! is refused. A named file that cannot be read fails the build, since an
//! image that silently lost its keyring would look like one without it.

use std::path::PathBuf;
use std::{env, fs, process};

fn main() {
    println!("cargo:rerun-if-env-changed=NONOS_PACMAN_KEYRING");
    for var in
        ["NONOS_PACMAN_MIRROR", "NONOS_PACMAN_HOST", "NONOS_PACMAN_PATH", "NONOS_PACMAN_REPOS"]
    {
        println!("cargo:rerun-if-env-changed={var}");
    }
    let Some(out) = env::var_os("OUT_DIR").map(PathBuf::from) else {
        eprintln!("no OUT_DIR");
        process::exit(1);
    };
    let ring = match env::var("NONOS_PACMAN_KEYRING") {
        Ok(path) => {
            println!("cargo:rerun-if-changed={path}");
            fs::read(&path).unwrap_or_else(|e| {
                eprintln!("NONOS_PACMAN_KEYRING={path}: {e}");
                process::exit(1)
            })
        }
        Err(_) => Vec::new(),
    };
    if let Err(e) = fs::write(out.join("pacman-keyring.gpg"), ring) {
        eprintln!("writing the keyring: {e}");
        process::exit(1);
    }
}
