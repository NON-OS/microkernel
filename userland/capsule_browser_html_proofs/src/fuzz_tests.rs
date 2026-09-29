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

//! A fixed-seed fuzz pass small enough for every run: interacting markup
//! pieces glued at random, held to the arena invariants and the depth cap.

use crate::browser::dom::limits::MAX_DEPTH;
use crate::browser::dom::{parse, parse_fragment};
use crate::shape::{check, depth};

/// The pieces, split at "|". A line break inside the literal is skipped.
const PIECES: &[u8] = b"<table>|<tr>|<td>|</table>|<caption>|<colgroup><col>|x| |<b>|</b>|<i>|\
    <a href=1>|</a>|<nobr>|<p>|</p>|<div>|</div>|<svg>|</svg>|<math><mi>|<foreignObject>|\
    <![CDATA[c]]>|<select><option>|<template>|</template>|<frameset>|<body a=1>|<html b=2>|\
    <title>t|<script><!--<script>|</script>|<textarea>\n|<plaintext>|<!--|-->|&amp|&#x80;|\
    \xFF\xC3|\0|\r\n|<li>|<dd>|<h1>|<h2>|<form>|</form>|<input type=hidden>|<image>|<br/>|\
    </br>|<button>|<ruby><rt>|<isindex>";
const CONTEXTS: &[&str] = &["div", "tbody", "tr", "td", "select", "title", "svg svg", "table"];

struct Rng(u64);

impl Rng {
    fn next(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}

#[test]
fn random_markup_always_builds_a_sound_tree() {
    let pieces: Vec<&[u8]> = PIECES.split(|&c| c == b'|').collect();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for round in 0..3_000 {
        let count = 1 + rng.next(400);
        let mut input = Vec::new();
        for _ in 0..count {
            input.extend_from_slice(pieces[rng.next(pieces.len())]);
        }
        let d = parse(&input);
        check(&d);
        assert!(depth(&d) <= MAX_DEPTH + 1, "round {round}");
        let f = parse_fragment(&input, CONTEXTS[round % CONTEXTS.len()]);
        check(&f);
    }
}

#[test]
fn deep_random_nesting_stays_under_the_cap() {
    let mut rng = Rng(777);
    let open: &[&[u8]] = &[b"<div>", b"<b>", b"<table><td>", b"<svg><g>", b"<ul><li>", b"<a>"];
    for _ in 0..20 {
        let mut input = Vec::new();
        for _ in 0..3_000 {
            input.extend_from_slice(open[rng.next(open.len())]);
        }
        let d = parse(&input);
        check(&d);
        assert!(depth(&d) <= MAX_DEPTH + 1, "depth {}", depth(&d));
    }
}
