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

//! The list a select opens: its options, drawn by the browser over the
//! page (paint::select_list), chosen with the pointer or with Up, Down,
//! Home, End, Enter and Esc.
//!
//! A select showed its first option and nothing else: the reader could not
//! choose another, so a search filter, a country or a sort order stayed
//! where the page put it. What it holds and where it goes are decided here,
//! apart from the window, so they are proved on the host.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::dom::node::{Node, NodeKind};
use crate::browser::dom::Dom;

/// One row: an option, or the heading of an optgroup, which is not chosen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// The option's node, or the optgroup's for a heading.
    pub id: usize,
    pub label: String,
    /// Disabled, or itself in a disabled optgroup.
    pub disabled: bool,
    pub heading: bool,
    /// Selected when the list opened, or chosen since.
    pub selected: bool,
}

impl Row {
    /// Whether the reader can choose it: an option, not disabled.
    pub fn choosable(&self) -> bool {
        !self.disabled && !self.heading
    }
}

/// Rows shown at once; more scroll within the list.
pub const MAX_ROWS: usize = 12;
/// The height of one row, in pixels.
pub const ROW_H: u32 = 22;

/// An open list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectList {
    /// The select it belongs to.
    pub select: usize,
    pub rows: Vec<Row>,
    /// The highlighted row, which Enter chooses.
    pub hi: usize,
    /// The first row shown.
    pub first: usize,
    /// A `multiple` select: a choice toggles and the list stays open.
    pub multiple: bool,
    /// The widest label in pixels, measured once by the window when the
    /// list opens (0 until then).
    pub widest: u32,
}

impl SelectList {
    /// The list for `select`, highlighting the option it shows. None for
    /// anything but a select, a disabled one, or one with nothing to choose.
    pub fn open(dom: &Dom, select: usize) -> Option<SelectList> {
        let s = dom.nodes.get(select)?;
        if s.kind != NodeKind::Element || s.tag != "select" || s.attr("disabled").is_some() {
            return None;
        }
        let mut rows = Vec::new();
        for &c in &s.children {
            let Some(n) = dom.nodes.get(c).filter(|n| n.kind == NodeKind::Element) else {
                continue;
            };
            match n.tag.as_str() {
                "option" => rows.push(option_row(dom, c, false)),
                "optgroup" => {
                    let off = n.attr("disabled").is_some();
                    let label = n.attr("label").unwrap_or("");
                    rows.push(Row {
                        id: c,
                        label: String::from(label),
                        disabled: true,
                        heading: true,
                        selected: false,
                    });
                    for &o in &n.children {
                        let is_option = dom
                            .nodes
                            .get(o)
                            .is_some_and(|e| e.kind == NodeKind::Element && e.tag == "option");
                        if is_option {
                            rows.push(option_row(dom, o, off));
                        }
                    }
                }
                _ => {}
            }
        }
        let options = rows.iter().filter(|r| !r.heading);
        let shown = options.clone().find(|r| r.selected).or(options.clone().next())?.id;
        let hi = rows.iter().position(|r| r.id == shown).unwrap_or(0);
        let multiple = s.attr("multiple").is_some();
        let mut list = SelectList { select, rows, hi, first: 0, multiple, widest: 0 };
        list.keep_in_view();
        Some(list)
    }

    /// Move the highlight `delta` choosable rows down (negative: up),
    /// stopping at either end.
    pub fn step(&mut self, delta: i32) {
        let down = delta > 0;
        for _ in 0..delta.unsigned_abs() {
            let next = match down {
                true => (self.hi + 1..self.rows.len()).find(|&i| self.rows[i].choosable()),
                false => (0..self.hi).rev().find(|&i| self.rows[i].choosable()),
            };
            match next {
                Some(i) => self.hi = i,
                None => break,
            }
        }
        self.keep_in_view();
    }

    /// Highlight the first choosable row, or the last.
    pub fn to_end(&mut self, last: bool) {
        let mut ids = (0..self.rows.len()).filter(|&i| self.rows[i].choosable());
        let at = match last {
            true => ids.next_back(),
            false => ids.next(),
        };
        if let Some(i) = at {
            self.hi = i;
        }
        self.keep_in_view();
    }

    /// The option the highlighted row stands for, if it can be chosen.
    pub fn highlighted(&self) -> Option<usize> {
        self.rows.get(self.hi).filter(|r| r.choosable()).map(|r| r.id)
    }

    /// Read again which options are selected, after a choice.
    pub fn refresh(&mut self, dom: &Dom) {
        for r in self.rows.iter_mut().filter(|r| !r.heading) {
            r.selected = dom.nodes.get(r.id).is_some_and(|n| n.attr("selected").is_some());
        }
    }

    /// How many rows are shown at once.
    pub fn shown(&self) -> usize {
        self.rows.len().min(MAX_ROWS)
    }

    fn keep_in_view(&mut self) {
        let n = self.shown();
        if self.hi < self.first {
            self.first = self.hi;
        } else if self.hi >= self.first + n {
            self.first = self.hi + 1 - n;
        }
    }
}

/// Where the list is drawn, in page-area coordinates (0 is the top of the
/// page area on screen).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

/// Place the list for a select laid out at `rect` (`[x, y, w, h]` in page
/// coordinates) on a page scrolled `scroll` down, in a page area `view`
/// wide and tall: under the select, or over it when it fits there and not
/// under, and inside the window across, wide enough for its widest label.
pub fn place(list: &SelectList, rect: [i32; 4], scroll: u32, view: (u32, u32)) -> Placed {
    let widest = list.widest;
    let h = list.shown() as u32 * ROW_H + 2;
    let top = rect[1] - scroll as i32;
    let under = top + rect[3];
    let fits_under = under + h as i32 <= view.1 as i32;
    let y = match fits_under || top < h as i32 {
        true => under,
        false => top - h as i32,
    };
    let w = (rect[2].max(0) as u32).max(widest + 24).min(view.0.max(1));
    let x = rect[0].min(view.0 as i32 - w as i32).max(0);
    Placed { x, y, w, h }
}

/// The row under page-area point (`x`, `y`), if the point is on the list.
pub fn row_at(list: &SelectList, at: Placed, x: i32, y: i32) -> Option<usize> {
    let inside = x >= at.x && x < at.x + at.w as i32 && y > at.y && y < at.y + at.h as i32 - 1;
    if !inside {
        return None;
    }
    let i = list.first + ((y - at.y - 1) as u32 / ROW_H) as usize;
    (i < list.rows.len()).then_some(i)
}

fn option_row(dom: &Dom, id: usize, group_off: bool) -> Row {
    let n = &dom.nodes[id];
    Row {
        id,
        label: option_label(dom, n),
        disabled: group_off || n.attr("disabled").is_some(),
        heading: false,
        selected: n.attr("selected").is_some(),
    }
}

/* What an option shows: its label attribute, else its text with white
 * space collapsed. */
fn option_label(dom: &Dom, n: &Node) -> String {
    if let Some(l) = n.attr("label").filter(|l| !l.is_empty()) {
        return String::from(l);
    }
    let mut text = String::new();
    for t in n.children.iter().filter_map(|&c| dom.nodes.get(c)) {
        if t.kind == NodeKind::Text {
            text.push_str(&t.text);
        }
    }
    let words: Vec<&str> = text.split_whitespace().collect();
    words.join(" ")
}
