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

//! GIF fixtures decode to their reference rasters, and the fuzz inputs that
//! once took a second (a tall interlaced frame on a tiny screen) take none.
use nonos_toolkit::image::gif::decode_gif_argb8888;
use std::time::Instant;

use crate::fixtures::{expectations, fnv, read};

#[test]
fn tall_interlaced_frames_on_small_screens_cost_nothing() {
    let mut names = std::vec![std::string::String::from("misc/gif_interlace_1x65535_43B.gif")];
    names.extend(["101532", "121368", "128535"].map(|n| std::format!("misc/slow_gif_1_{n}.gif")));
    for name in names {
        let t = Instant::now();
        let _ = decode_gif_argb8888(&read(&name), &mut [0u32; 64]);
        assert!(t.elapsed().as_millis() < 20, "{name} took {:?}", t.elapsed());
    }
}

#[test]
fn gif_fixtures_match_their_reference_decode() {
    for (name, want) in expectations().into_iter().filter(|e| e.0.ends_with(".gif")) {
        let (w, h, hash) = want.expect("reference decodes");
        let mut out = std::vec![0u32; (w * h) as usize];
        decode_gif_argb8888(&read(&name), &mut out).unwrap();
        assert_eq!(fnv(&out), hash, "{name}");
    }
}
