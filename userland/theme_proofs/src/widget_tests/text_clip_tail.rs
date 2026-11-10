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

//! `clip_tail`: the run unchanged, or its longest char-aligned end that fits.

use nonos_toolkit::widgets::text::{clip_tail, clip_tail_by, tail_by, width};

use super::samples::{sizes, tens, RUNS};
use super::serial::serial;

/// What is wrong with `out` as `clip_tail(s, max_w, px)`, if anything.
fn fault(s: &str, max_w: u32, px: f32, out: &str) -> Option<String> {
    let w = width(out, px);
    if w > max_w {
        return Some(format!("{out:?} is {w} wide, over {max_w}"));
    }
    if width(s, px) <= max_w {
        return (out != s).then(|| format!("{out:?} changed a run that fits"));
    }
    let from = s.len() - out.len();
    if !s.ends_with(out) || !s.is_char_boundary(from) || out.starts_with('…') {
        return Some(format!("{out:?} is not a bare char-aligned end"));
    }
    let prev = s[..from].chars().next_back().map(|c| format!("{c}{out}"));
    let longer = prev.filter(|p| width(p, px) <= max_w);
    longer.map(|p| format!("{out:?} stops short of {p:?}"))
}

#[test]
fn clip_tail_keeps_the_end_and_fits() {
    let _g = serial();
    let mut fails = Vec::new();
    for px in sizes() {
        for s in RUNS {
            for max_w in 0..=width(s, px) + 4 {
                let out = clip_tail(s, max_w, px);
                let why = fault(s, max_w, px, out);
                fails.extend(why.map(|w| format!("{s:?} at {px} in {max_w}: {w}")));
            }
        }
    }
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}

#[test]
fn clip_tail_by_keeps_the_longest_end_that_fits() {
    assert_eq!(clip_tail_by("abcdef", 60, tens), "abcdef");
    assert_eq!(clip_tail_by("abcdef", 59, tens), "bcdef");
    assert_eq!(clip_tail_by("日本語です", 35, tens), "語です");
    assert_eq!(clip_tail_by("üü", 9, tens), "");
}

#[test]
fn clip_tail_by_keeps_what_tail_by_keeps_less_room_for_the_ellipsis() {
    for s in RUNS {
        for max_w in 10..tens(s) {
            let end = tail_by(s, max_w, tens);
            let kept = clip_tail_by(s, max_w - 10, tens);
            assert_eq!(end.strip_prefix('…'), Some(kept), "{s:?} in {max_w}");
        }
    }
}
