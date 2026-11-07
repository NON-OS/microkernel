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
//! The Launchpad's page tabs light only the tab under the pointer and a
//! click names the page whose tab it lands in.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Pointer, Rect, Tabs};

use crate::shell_layer::layer;
use crate::shell_page_tabs::{draw, labels, lit, place};
use crate::ui_font::px;

const LAYER: Rect = Rect { x: 0, y: 0, w: 1920, h: 200 };

fn painted(tabs: Tabs, at: Rect, p: &Pointer) -> Vec<u32> {
    let (w, h) = (LAYER.w, LAYER.h);
    let mut words = vec![roles().bg.argb(); (w * h) as usize];
    draw(&mut unsafe { layer(words.as_mut_ptr() as u64, w * 4, w, h) }, tabs, at, p);
    words
}

fn moved_only_in(a: &[u32], b: &[u32], owns: impl Fn(i32, i32) -> bool) -> bool {
    let at = |i: usize| ((i as u32 % LAYER.w) as i32, (i as u32 / LAYER.w) as i32);
    let mut moved = (0..a.len()).filter(|&i| a[i] != b[i]).peekable();
    moved.peek().is_some() && moved.all(|i| owns(at(i).0, at(i).1))
}

pub(crate) fn page_tabs_select_where_they_are_clicked(tag: &str) {
    let names = labels(4);
    let words: Vec<&str> = names.iter().map(String::as_str).collect();
    let tabs = |sel| Tabs::new(&words, sel);
    let at = place(&tabs(0), LAYER.w, LAYER.h, px(44));
    assert!(at.x + at.w <= LAYER.w && at.y + at.h == LAYER.h, "{tag}: off the band");
    let spot = |i| {
        let on: Vec<_> = (at.x..at.x + at.w)
            .flat_map(|x| (at.y..at.y + at.h).map(move |y| (x as i32, y as i32)))
            .filter(|&(x, y)| tabs(0).hit(at, x, y) == Some(i))
            .collect();
        on[on.len() / 2]
    };
    let quiet = Pointer::new();
    for page in 0..4 {
        let ((x, y), mut p, other) = (spot(page), Pointer::new(), (page + 1) % 4);
        assert_eq!(tabs(0).hit(at, x, y), Some(page), "{tag}: click {page}");
        let (chosen, plain) = (painted(tabs(page), at, &quiet), painted(tabs(other), at, &quiet));
        let owns = |x, y| matches!(tabs(0).hit(at, x, y), Some(t) if t == page || t == other);
        assert!(moved_only_in(&plain, &chosen, owns), "{tag}: page {page} off its tab");
        p.moved(x, y);
        assert_eq!(lit(&tabs(other), at, &p), Some(page), "{tag}: hover {page}");
        let lit_in = |x, y| tabs(0).hit(at, x, y) == Some(page);
        let hovered = painted(tabs(other), at, &p);
        assert!(moved_only_in(&plain, &hovered, lit_in), "{tag}: hover {page} off its tab");
        p.moved(x, at.y as i32 - 1);
        assert_eq!(lit(&tabs(other), at, &p), None, "{tag}: left the tabs");
        assert_eq!(painted(tabs(other), at, &p), plain, "{tag}: left lit");
    }
}
