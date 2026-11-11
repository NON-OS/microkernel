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

//! The installer's paragraphs broken by `wrap_by` where its old `paragraph` broke them.

use nonos_toolkit::widgets::text::wrap_by;

use crate::installer::text::body_w;
use crate::installer_layout_tests::bodies;
use crate::literals::literals;
use crate::{measure, Face};

const SAMPLES: [&str; 3] =
    ["caf\u{e9} \u{fc}n\u{ef}code \u{2713}", "anoverlongwordwithnospaces in a line", ""];

fn old_paragraph(max_w: u32, s: &str, px: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in s.split(' ') {
        let candidate = if current.is_empty() {
            String::from(word)
        } else {
            alloc::format!("{current} {word}")
        };
        if measure(&candidate, Face::Body, px, 0.0) > max_w && !current.is_empty() {
            lines.push(current);
            current = String::from(word);
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn paragraphs() -> Vec<String> {
    let spaced = |t: &String| !t.contains("  ") && t.trim() == t && !t.contains(['\t', '\n', '\r']);
    let all = literals("../capsule_install/src/install/ui", false).into_iter().map(|l| l.text);
    all.filter(|t| t.contains(' ') && spaced(t)).chain(SAMPLES.map(String::from)).collect()
}

#[test]
fn every_paragraph_breaks_where_the_old_paragraph_broke_it() {
    let mut sizes: Vec<f32> = bodies().iter().map(|b| b.m.body_px).collect();
    sizes.sort_by(f32::total_cmp);
    sizes.dedup();
    let all = paragraphs();
    assert!(all.len() > 40, "{}", all.len());
    for s in &all {
        for &px in &sizes {
            for w in 0..=measure(s, Face::Body, px, 0.0) + 4 {
                let new: Vec<&str> =
                    wrap_by(s, w as i32, body_w(px)).into_iter().map(|(a, e)| &s[a..e]).collect();
                assert_eq!(new, old_paragraph(w, s, px), "{s:?} at {px} in {w}");
            }
        }
    }
}
