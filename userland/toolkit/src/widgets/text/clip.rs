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

//! Cutting a run to a width with no mark, borrowed from the run itself.

use alloc::vec::Vec;

use crate::paint::measure_ttf;

/// `s` unchanged when it measures within `max_w`, otherwise the longest
/// char-aligned prefix of `s` that still fits, found by halving as
/// `truncate_by` finds its prefix, with no ellipsis added. Empty when not even
/// the first char fits.
pub fn clip_by(s: &str, max_w: i32, measure: impl Fn(&str) -> i32) -> &str {
    if measure(s) <= max_w {
        return s;
    }
    let ends: Vec<usize> = s.char_indices().map(|(i, c)| i + c.len_utf8()).collect();
    let (mut lo, mut hi) = (0usize, ends.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if measure(&s[..ends[mid - 1]]) <= max_w {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    &s[..if lo == 0 { 0 } else { ends[lo - 1] }]
}

/// `s` unchanged when it measures within `max_w`, otherwise the longest
/// char-aligned end of `s` that still fits, found by halving as `tail_by`
/// finds its end, with no ellipsis in front. Empty when not even the last char
/// fits.
pub fn clip_tail_by(s: &str, max_w: i32, measure: impl Fn(&str) -> i32) -> &str {
    if measure(s) <= max_w {
        return s;
    }
    let starts: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
    let (mut lo, mut hi) = (0usize, starts.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if measure(&s[starts[mid]..]) <= max_w {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    &s[starts.get(lo).copied().unwrap_or(s.len())..]
}

/// `clip_by` measured in the body face at `px`, the face `draw::text` paints
/// with.
pub fn clip(s: &str, max_w: u32, px: f32) -> &str {
    clip_by(s, i32::try_from(max_w).unwrap_or(i32::MAX), |t| measure_ttf(t, px))
}

/// `clip_tail_by` measured in the body face at `px`, the face `draw::text`
/// paints with.
pub fn clip_tail(s: &str, max_w: u32, px: f32) -> &str {
    clip_tail_by(s, i32::try_from(max_w).unwrap_or(i32::MAX), |t| measure_ttf(t, px))
}
