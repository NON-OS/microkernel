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

//! Query buffer, selection and hiding.

use crate::palette::filter::filter;
use crate::palette::state::{Palette, QUERY_CAP};
use crate::support::index;

#[test]
fn a_query_longer_than_the_buffer_never_panics() {
    let mut p = Palette::new();
    p.show();
    for i in 0..QUERY_CAP * 4 {
        p.push(b'a' + (i % 26) as u8);
    }
    assert_eq!(p.qlen, QUERY_CAP);
    assert_eq!(p.needle().len(), QUERY_CAP);
    let mut out = [0usize; 16];
    assert_eq!(filter(&index(), p.needle(), &mut out), 0);
    for _ in 0..QUERY_CAP * 4 {
        p.backspace();
    }
    assert_eq!(p.qlen, 0);
    assert_eq!(filter(&index(), p.needle(), &mut out), index().len());
}

#[test]
fn selection_wraps_and_survives_an_empty_result_set() {
    let mut p = Palette::new();
    p.step(-1, 0);
    assert_eq!(p.sel, 0);
    p.step(-1, 4);
    assert_eq!(p.sel, 3);
    p.step(1, 4);
    assert_eq!(p.sel, 0);
    p.push(b'x');
    assert_eq!(p.sel, 0);
}

#[test]
fn hiding_clears_the_query_so_the_next_open_starts_empty() {
    let mut p = Palette::new();
    p.show();
    p.push(b'g');
    p.step(1, 3);
    p.hide();
    assert!(!p.open);
    assert_eq!(p.needle(), b"");
    assert_eq!(p.sel, 0);
}
