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

//! Every vector, what made it, and the input regenerated.

use crate::kinds::{mixed, noise, runs, text};
use crate::rng::Rng;

pub const TABLE: [(&str, &str, u64, usize); 12] = [
    ("empty", "text", 1, 0),
    ("tiny", "text", 2, 50),
    ("text64k", "text", 3, 65536),
    ("text300k", "text", 4, 300000),
    ("noise140k", "noise", 5, 140000),
    ("zeros300k", "zeros", 6, 300000),
    ("runs100k", "runs", 7, 100000),
    ("mixed256k", "mixed", 8, 262144),
    ("nocheck", "text", 9, 20000),
    ("stream", "text", 10, 50000),
    ("fast", "text", 11, 100000),
    ("long", "text", 12, 400000),
];

pub fn input(kind: &str, seed: u64, size: usize) -> Vec<u8> {
    let r = &mut Rng::new(seed);
    match kind {
        "text" => text(r, size),
        "noise" => noise(r, size),
        "runs" => runs(r, size),
        "mixed" => mixed(r, size),
        _ => vec![0; size],
    }
}
