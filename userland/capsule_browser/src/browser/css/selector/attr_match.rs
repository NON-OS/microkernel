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

use super::attr::{is_space, AttrOp, AttrTest};

impl AttrTest {
    pub fn matches(&self, have: &str) -> bool {
        let (h, v, ci) = (have.as_bytes(), self.value.as_bytes(), self.case_insensitive);
        let n = v.len();
        match self.op {
            AttrOp::Present => true,
            AttrOp::Eq => same(h, v, ci),
            AttrOp::Contains => n > 0 && contains(have, &self.value, ci),
            AttrOp::Starts => n > 0 && h.len() >= n && same(&h[..n], v, ci),
            AttrOp::Ends => n > 0 && h.len() >= n && same(&h[h.len() - n..], v, ci),
            /* Words split on CSS whitespace; an empty value or one holding
             * whitespace can never equal a word. */
            AttrOp::Word => h.split(|b| is_space(*b)).any(|w| !w.is_empty() && same(w, v, ci)),
            AttrOp::Lang => same(h, v, ci) || (h.len() > n && h[n] == b'-' && same(&h[..n], v, ci)),
        }
    }

    /* The most bytes `matches` reads for `have`, so the matcher can pay
     * for a test before it runs: the value's length for the tests anchored
     * at one place, the whole attribute for the two that search it. */
    pub fn cost(&self, have: &str) -> usize {
        let (h, v) = (have.len(), self.value.len());
        match self.op {
            AttrOp::Present => 0,
            AttrOp::Eq | AttrOp::Starts | AttrOp::Ends => v,
            AttrOp::Lang => 2 * v,
            AttrOp::Word => h + v,
            AttrOp::Contains if self.case_insensitive => 2 * (h + v),
            AttrOp::Contains => h + v,
        }
    }
}

fn same(a: &[u8], b: &[u8], ci: bool) -> bool {
    if ci {
        a.eq_ignore_ascii_case(b)
    } else {
        a == b
    }
}

/* Substring search in time linear in both strings: core's two-way search,
 * over ASCII-lowered copies for the i flag. Lowering only ASCII keeps both
 * valid UTF-8 and compares every other character exactly, as before. */
fn contains(have: &str, value: &str, ci: bool) -> bool {
    if ci {
        have.to_ascii_lowercase().contains(value.to_ascii_lowercase().as_str())
    } else {
        have.contains(value)
    }
}
