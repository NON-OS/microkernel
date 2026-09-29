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

use alloc::string::String;

/* The operator of an attribute selector: presence, or one of the six value
 * tests (=, *=, ^=, $=, ~=, |=). */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttrOp {
    Present,
    Eq,
    Contains,
    Starts,
    Ends,
    Word,
    Lang,
}

/* One attribute test. The `i` flag asks for an ASCII case-insensitive
 * comparison of the value; without it the value compares exactly. */
#[derive(Clone)]
pub struct AttrTest {
    pub op: AttrOp,
    pub value: String,
    pub case_insensitive: bool,
}

impl AttrTest {
    pub fn matches(&self, have: &str) -> bool {
        let (h, v, ci) = (have.as_bytes(), self.value.as_bytes(), self.case_insensitive);
        let n = v.len();
        match self.op {
            AttrOp::Present => true,
            AttrOp::Eq => same(h, v, ci),
            AttrOp::Contains => n > 0 && h.windows(n).any(|w| same(w, v, ci)),
            AttrOp::Starts => n > 0 && h.len() >= n && same(&h[..n], v, ci),
            AttrOp::Ends => n > 0 && h.len() >= n && same(&h[h.len() - n..], v, ci),
            /* Words split on CSS whitespace; an empty value or one holding
             * whitespace can never equal a word. */
            AttrOp::Word => h.split(|b| is_space(*b)).any(|w| !w.is_empty() && same(w, v, ci)),
            AttrOp::Lang => same(h, v, ci) || (h.len() > n && h[n] == b'-' && same(&h[..n], v, ci)),
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

/* CSS whitespace: space, tab, line feed, carriage return, form feed. */
pub fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0c)
}
