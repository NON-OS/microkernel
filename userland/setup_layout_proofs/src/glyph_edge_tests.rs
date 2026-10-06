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

//! Every capital the labels draw keeps its shape at every pen phase. A glyph
//! edge on a whole pixel used to break the rasteriser's running sums: the
//! T in FIRST-BOOT SETUP and NETWORK ROUTE came out as broken bars.

use crate::faces::{font, Face};
use nonos_toolkit::font::ttf::draw_text_tracked;

const W: u32 = 160;
const H: u32 = 32;

/* Coverage of `s` drawn as the labels draw it, mono tracked at 0.3 em, with
`nudge` px more tracking, one byte per pixel. The surface is opaque black,
as the setup screens' are, so the blue channel reads back the coverage. */
fn draw(s: &str, px: f32, nudge: f32) -> Vec<u8> {
    let f = font(Face::Mono).expect("the mono face parses");
    let mut buf = vec![0xff00_0000u32; (W * H) as usize];
    draw_text_tracked(&f, &mut buf, W as usize, W, H, 2, 2, s, 0xffff_ffff, px, px * 0.3 + nudge);
    buf.iter().map(|p| (p & 0xff) as u8).collect()
}

/* The last glyph of `lead` + `ch`, alone: the run drawn, less the lead drawn
by itself. A nudge of 0, 1/4, 1/2 and 3/4 px puts it at every phase. */
fn last(lead: &str, ch: char, px: f32, nudge: f32) -> Vec<u8> {
    let both = draw(&format!("{lead}{ch}"), px, nudge);
    let only = draw(lead, px, nudge);
    both.iter().zip(&only).map(|(&a, &b)| a.saturating_sub(b)).collect()
}

const NUDGES: [f32; 4] = [0.0, 0.25, 0.5, 0.75];

fn total(c: &[u8]) -> u32 {
    c.iter().map(|&v| v as u32).sum()
}

/* Label sizes from 9 to 20 px in quarter pixels: the screens scale theirs,
so an edge can land on a whole pixel at any of them. */
fn sizes() -> impl Iterator<Item = f32> {
    (36..=80).map(|q| q as f32 / 4.0)
}

/* The runs of ink in each row of the glyph below its top two, where a T is
its stem alone: one run per row. A broken raster shows two bars. */
fn stem_runs(c: &[u8]) -> Vec<usize> {
    let rows: Vec<&[u8]> = c.chunks(W as usize).filter(|r| r.iter().any(|&v| v > 32)).collect();
    rows.iter()
        .skip(2)
        .take(rows.len().saturating_sub(3))
        .map(|r| r.windows(2).filter(|p| p[0] <= 32 && p[1] > 32).count() + usize::from(r[0] > 32))
        .collect()
}

// The pairs the screens showed broken, and a plain one.
const LEADS: [&str; 5] = ["O", "U", "S", "ROU", "BOO"];

#[test]
fn capitals_keep_their_ink_at_every_phase() {
    let mut bad = Vec::new();
    for px in sizes() {
        for ch in 'A'..='Z' {
            let alone = total(&draw(&ch.to_string(), px, 0.0));
            for (lead, nudge) in LEADS.iter().flat_map(|l| NUDGES.map(|n| (*l, n))) {
                let got = total(&last(lead, ch, px, nudge));
                if got.abs_diff(alone) * 5 > alone {
                    bad.push(format!("{ch} after {lead} +{nudge} at {px} px: ink {got}, alone {alone}"));
                }
            }
        }
    }
    assert!(bad.is_empty(), "glyphs that lost their shape:\n{}", bad.join("\n"));
}

#[test]
fn the_t_keeps_one_stem() {
    for px in sizes() {
        for (lead, nudge) in LEADS.iter().flat_map(|l| NUDGES.map(|n| (*l, n))) {
            let runs = stem_runs(&last(lead, 'T', px, nudge));
            assert!(!runs.is_empty(), "T after {lead} +{nudge} at {px} px: no stem");
            assert!(runs.iter().all(|&n| n == 1), "T after {lead} +{nudge} at {px} px: stem rows {runs:?}");
        }
    }
}


