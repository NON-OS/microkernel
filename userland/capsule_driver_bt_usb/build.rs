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

//! With the `realtek` feature, the Realtek files are built into the
//! capsule. Before anything is compiled, each file named in `src/rtl`'s
//! pins must be in `nonos-bootloader/firmware/rtl_bt/` with exactly its
//! pinned length and SHA-256, and `src/run/files.rs` must embed exactly
//! those files: a build cannot seal a blob the pins do not name.

use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::{env, fs, path::Path};

const PINS: [&str; 2] = ["src/rtl/pins.rs", "src/rtl/pins_config.rs"];
const FILES: &str = "src/run/files.rs";
const DIR: &str = "../../nonos-bootloader/firmware/rtl_bt";

fn main() {
    PINS.iter().chain([&FILES]).for_each(|p| println!("cargo:rerun-if-changed={p}"));
    if env::var_os("CARGO_FEATURE_REALTEK").is_none() {
        return;
    }
    let root = env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let read = |p: &str| fs::read_to_string(Path::new(&root).join(p)).expect(p);
    let mut pinned = BTreeSet::new();
    for (file, len, sha256) in PINS.iter().flat_map(|p| pins(&read(p))) {
        let path = Path::new(&root).join(DIR).join(&file);
        println!("cargo:rerun-if-changed={}", path.display());
        let Ok(bytes) = fs::read(&path) else {
            panic!("{}: missing; the owner adds it (Decision 19)", path.display());
        };
        assert_eq!(bytes.len(), len, "{file}: not the pinned length");
        let digest: String = Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(digest, sha256, "{file}: not the pinned SHA-256");
        pinned.insert(file);
    }
    let embedded: BTreeSet<String> = quoted(&read(FILES)).into_iter().collect();
    assert_eq!(pinned, embedded, "files.rs must embed exactly the pinned files");
}

/// Each `Pin`'s file, length and SHA-256 as its source writes them.
fn pins(src: &str) -> Vec<(String, usize, String)> {
    let field = |rest: &str, key: &str| rest.split(key).nth(1).map(str::to_owned);
    src.split("file: \"")
        .skip(1)
        .map(|rest| {
            let file = rest.split('"').next().expect("a quoted file name").to_owned();
            let len = field(rest, "len: ").expect("a len after each file");
            let len = len.split(',').next().unwrap().replace('_', "").trim().parse().expect("len");
            let sha = field(rest, "hex32(\"").expect("a digest after each file");
            (file, len, sha.split('"').next().unwrap().to_owned())
        })
        .collect()
}

/// The file names files.rs lists, one quoted name a line.
fn quoted(src: &str) -> Vec<String> {
    src.lines()
        .map(str::trim)
        .filter_map(|l| l.strip_prefix('"')?.strip_suffix("\",").map(str::to_owned))
        .collect()
}
