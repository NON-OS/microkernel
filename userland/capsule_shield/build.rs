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

//! The periodic cache the prover proves from, embedded when the build names
//! it: the Nix build sets NONOS_PERIODIC_CACHE to the file its
//! periodic-cache derivation made and checked. A build without one embeds
//! nothing, and the first proof builds the cache and keeps it, as before.

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=NONOS_PERIODIC_CACHE");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"))
        .join("periodic.top");
    let bytes = match std::env::var_os("NONOS_PERIODIC_CACHE") {
        Some(path) => {
            println!("cargo:rerun-if-changed={}", PathBuf::from(&path).display());
            std::fs::read(&path).expect("NONOS_PERIODIC_CACHE names a file that cannot be read")
        }
        None => Vec::new(),
    };
    std::fs::write(out, bytes).expect("OUT_DIR is writable");
}
