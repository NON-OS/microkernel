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

//! Seeds for fuzz/pack_parse, written rather than kept: packs of a few
//! shapes a package takes, and each cut short at every 97th byte.
//!
//!     cargo run --example seeds -- fuzz/corpus/pack_parse

use std::{env, fs, path::PathBuf};

use nonos_linux_pack::{build, Kind, Source};

fn src<'a>(name: &str, kind: Kind, exec: bool, bytes: &'a [u8]) -> Source<'a> {
    Source { name: name.as_bytes().to_vec(), kind, exec, bytes }
}

fn shapes(bodies: &[Vec<u8>]) -> Vec<Vec<Source<'_>>> {
    let tree = |n: usize| -> Vec<Source> {
        let mut v =
            vec![src("usr", Kind::Dir, false, b""), src("usr/share", Kind::Dir, false, b"")];
        let file = |i: usize| src(&format!("usr/share/f{i:04}"), Kind::File, false, &bodies[i]);
        v.extend((0..n).map(file));
        v
    };
    vec![
        Vec::new(),
        vec![src("usr/bin/age", Kind::File, true, &[0x7f, b'E', b'L', b'F'])],
        vec![
            src("usr/bin/vi", Kind::Link, false, b"vim"),
            src("usr/bin/vim", Kind::File, true, b"x"),
        ],
        tree(40),
    ]
}

fn main() {
    let out = PathBuf::from(env::args().nth(1).expect("an output directory"));
    fs::create_dir_all(&out).expect("the directory");
    let bodies: Vec<Vec<u8>> = (0..40).map(|i| vec![i as u8; 70]).collect();
    for (n, shape) in shapes(&bodies).into_iter().enumerate() {
        let pack = build(shape).expect("a seed builds");
        for cut in (0..=pack.len()).step_by(97).chain([pack.len()]) {
            fs::write(out.join(format!("seed-{n}-{cut}")), &pack[..cut]).expect("written");
        }
    }
}
