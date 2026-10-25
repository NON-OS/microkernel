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

//! No boot path reads the firmware's BGRT logo: no kernel source names the
//! table's signature, a BGRT or BMP parser, or the logo it once copied. The
//! kernel reads every file under src/ as it stands.

use std::fs;
use std::path::{Path, PathBuf};

const GONE: [&str; 6] =
    ["b\"BGRT\"", "parse_bgrt", "boot_logo", "bmp_check", "bgrt_entry", "copy_logo"];

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).expect("src/ reads") {
        let p = e.expect("an entry").path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

#[test]
fn no_kernel_source_reads_the_bgrt_logo() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let mut files = Vec::new();
    sources(&root, &mut files);
    assert!(files.len() > 500, "the whole kernel tree is read: {}", files.len());
    for f in files {
        let text = fs::read_to_string(&f).expect("a source reads");
        for word in GONE {
            assert!(!text.contains(word), "{} names {word}", f.display());
        }
    }
}
