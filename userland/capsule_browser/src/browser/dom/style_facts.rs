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

//! What a script's `getComputedStyle` answers: the cascade's own result
//! for each node, kept from the last layout as `rects` are.
//!
//! It answered an empty string for every property, so code that asks
//! whether a menu is shown (`display`), how tall a panel is, or what colour
//! a theme set took the wrong branch on every page. The common properties
//! now come from the cascade the page is drawn with and the boxes it was
//! laid out in. A property not kept here answers what the element's own
//! style attribute gives, else empty (qjs_bridge::computed).

use alloc::format;
use alloc::string::String;
use alloc::vec;

use super::tree::Dom;

/// How the layout treats the box, as `display` reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Display {
    None,
    Inline,
    Block,
    InlineBlock,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    Table,
    TableRow,
    TableCell,
    Contents,
}

/// `position`, with fixed and sticky told apart from absolute and static.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pos {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

/// One node's cascade result, the part a script can ask for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Facts {
    pub display: Display,
    /// visibility: hidden or collapse on this element.
    pub hidden: bool,
    pub position: Pos,
    /// ARGB.
    pub color: u32,
    /// ARGB, 0 for transparent.
    pub bg: u32,
    pub font_px: f32,
    pub bold: bool,
    pub italic: bool,
    /// 0 to 255.
    pub opacity: u8,
    /// Top, right, bottom, left: margins in pixels, plus the per-mille of
    /// the containing block's width a percentage adds.
    pub margin: [i32; 4],
    pub margin_pml: [i32; 4],
    /// margin-left and margin-right were auto.
    pub margin_auto: [bool; 2],
    pub pad: [u32; 4],
    pub border: [u32; 4],
    /// box-sizing: border-box, so width and height read the border box.
    pub border_box: bool,
}

impl Facts {
    /// What a node with no style of its own reads.
    pub const NONE: Facts = Facts {
        display: Display::Inline,
        hidden: false,
        position: Pos::Static,
        color: 0xFF00_0000,
        bg: 0,
        font_px: 16.0,
        bold: false,
        italic: false,
        opacity: 255,
        margin: [0; 4],
        margin_pml: [0; 4],
        margin_auto: [false; 2],
        pad: [0; 4],
        border: [0; 4],
        border_box: false,
    };
}

/* Elements whose width a style sets though they sit in a line. */
const REPLACED: &[&str] =
    &["img", "input", "textarea", "select", "button", "canvas", "video", "iframe", "svg"];

impl Dom {
    /// Keep the cascade result of every node, by node id, from the layout
    /// just made.
    pub fn record_facts<F: Fn(usize) -> Facts>(&mut self, of: F) {
        self.facts = vec![Facts::NONE; self.nodes.len()];
        for (id, f) in self.facts.iter_mut().enumerate() {
            *f = of(id);
        }
    }

    /// `getComputedStyle(node).getPropertyValue(prop)` for the properties
    /// kept here (`prop` in its CSS spelling), resolved as browsers do:
    /// colours as rgb(), lengths in px, width and height as laid out.
    /// None for a property not kept, or a node with no facts yet.
    pub fn computed_value(&self, node: usize, prop: &str) -> Option<String> {
        let f = self.facts.get(node)?;
        let px = |v: i64| format!("{}px", v);
        let side = |i: usize| self.used_margin(node, i);
        Some(match prop {
            "display" => String::from(display_name(f.display)),
            "visibility" => String::from(if self.hidden_here(node) { "hidden" } else { "visible" }),
            "position" => String::from(pos_name(f.position)),
            "color" => rgb(f.color),
            "background-color" => rgb(f.bg),
            "font-size" => format!("{}px", trim(f.font_px)),
            "font-weight" => String::from(if f.bold { "700" } else { "400" }),
            "font-style" => String::from(if f.italic { "italic" } else { "normal" }),
            "opacity" => share(f.opacity),
            "box-sizing" => String::from(if f.border_box { "border-box" } else { "content-box" }),
            "width" => self.used_size(node, 2),
            "height" => self.used_size(node, 3),
            "margin-top" => px(side(0)),
            "margin-right" => px(side(1)),
            "margin-bottom" => px(side(2)),
            "margin-left" => px(side(3)),
            "margin" => four([side(0), side(1), side(2), side(3)]),
            "padding-top" => px(f.pad[0] as i64),
            "padding-right" => px(f.pad[1] as i64),
            "padding-bottom" => px(f.pad[2] as i64),
            "padding-left" => px(f.pad[3] as i64),
            "padding" => four(f.pad.map(|v| v as i64)),
            "border-top-width" => px(f.border[0] as i64),
            "border-right-width" => px(f.border[1] as i64),
            "border-bottom-width" => px(f.border[2] as i64),
            "border-left-width" => px(f.border[3] as i64),
            _ => return None,
        })
    }

    /* Hidden by its own visibility or an ancestor's, as visibility is
     * inherited (and as the page is drawn: a hidden box hides its whole
     * subtree here). */
    fn hidden_here(&self, node: usize) -> bool {
        let mut cur = node;
        for _ in 0..512 {
            match self.facts.get(cur) {
                Some(f) if f.hidden => return true,
                _ => {}
            }
            let up = self.nodes.get(cur).map_or(0, |n| n.parent);
            if cur == 0 || up == cur {
                return false;
            }
            cur = up;
        }
        false
    }

    /* width (2) or height (3) as laid out: the content box, or the border
     * box under border-box sizing; "auto" for a box not laid out and for
     * a plain inline box, which no width applies to. */
    fn used_size(&self, node: usize, which: usize) -> String {
        let f = &self.facts[node];
        let r = self.rects.get(node).copied().unwrap_or([0; 4]);
        let tag = self.nodes.get(node).map_or("", |n| n.tag.as_str());
        let inline = f.display == Display::Inline && !REPLACED.contains(&tag);
        if f.display == Display::None || r == [0; 4] || inline {
            return String::from("auto");
        }
        let (a, b) = if which == 2 { (3, 1) } else { (0, 2) };
        let edges = match f.border_box {
            true => 0,
            false => (f.pad[a] + f.pad[b] + f.border[a] + f.border[b]) as i64,
        };
        format!("{}px", (r[which] as i64 - edges).max(0))
    }

    /* Margin side `i` (top, right, bottom, left) as used: its pixels plus
     * its percentage of the parent's content width, and for an auto left
     * or right margin the room the layout left on that side. */
    fn used_margin(&self, node: usize, i: usize) -> i64 {
        let f = &self.facts[node];
        let parent = self.nodes.get(node).map_or(0, |n| n.parent);
        let p = self.facts.get(parent).copied().unwrap_or(Facts::NONE);
        let pr = self.rects.get(parent).copied().unwrap_or([0; 4]);
        let content_l = pr[0] as i64 + (p.border[3] + p.pad[3]) as i64;
        let content_w = pr[2] as i64 - (p.border[1] + p.border[3] + p.pad[1] + p.pad[3]) as i64;
        let r = self.rects.get(node).copied().unwrap_or([0; 4]);
        let laid = r != [0; 4] && pr != [0; 4];
        match (i, f.margin_auto) {
            (3, [true, _]) if laid => r[0] as i64 - content_l,
            (1, [_, true]) if laid => content_l + content_w - (r[0] + r[2]) as i64,
            (1 | 3, _) if f.margin_auto[(i == 1) as usize] => 0,
            _ => f.margin[i] as i64 + f.margin_pml[i] as i64 * content_w.max(0) / 1000,
        }
    }
}

fn display_name(d: Display) -> &'static str {
    match d {
        Display::None => "none",
        Display::Inline => "inline",
        Display::Block => "block",
        Display::InlineBlock => "inline-block",
        Display::Flex => "flex",
        Display::InlineFlex => "inline-flex",
        Display::Grid => "grid",
        Display::InlineGrid => "inline-grid",
        Display::Table => "table",
        Display::TableRow => "table-row",
        Display::TableCell => "table-cell",
        Display::Contents => "contents",
    }
}

fn pos_name(p: Pos) -> &'static str {
    match p {
        Pos::Static => "static",
        Pos::Relative => "relative",
        Pos::Absolute => "absolute",
        Pos::Fixed => "fixed",
        Pos::Sticky => "sticky",
    }
}

/* An ARGB colour as browsers serialize a computed one. */
fn rgb(argb: u32) -> String {
    let (a, r, g, b) = (argb >> 24, (argb >> 16) & 0xFF, (argb >> 8) & 0xFF, argb & 0xFF);
    match a {
        255 => format!("rgb({}, {}, {})", r, g, b),
        a => format!("rgba({}, {}, {}, {})", r, g, b, share(a as u8)),
    }
}

/* A share kept in 255ths (opacity, a colour's alpha), to the two decimals
 * that tell 255ths apart as an author wrote them: 0.5, not 0.498. */
fn share(v: u8) -> String {
    let s = format!("{:.2}", v as f32 / 255.0);
    String::from(s.trim_end_matches('0').trim_end_matches('.'))
}

/* A number with at most three decimals and no trailing zeros. */
fn trim(v: f32) -> String {
    let s = format!("{:.3}", v);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    String::from(s)
}

/* Four sides as their shorthand serializes: as few as say them all. */
fn four(v: [i64; 4]) -> String {
    let px = |i: usize| format!("{}px", v[i]);
    match v {
        [t, r, b, l] if t == r && r == b && b == l => px(0),
        [t, r, b, l] if t == b && r == l => format!("{} {}", px(0), px(1)),
        [_, r, _, l] if r == l => format!("{} {} {}", px(0), px(1), px(2)),
        _ => format!("{} {} {} {}", px(0), px(1), px(2), px(3)),
    }
}
