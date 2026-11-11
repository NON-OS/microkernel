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

use super::layout::{font, px};

// Control-strip metrics: the em size every control's label is measured and drawn
// at, the gap between controls, and the band all of them share. Every control in
// the header sits in this one band.
const TOOL_PX: f32 = 17.0;
const TOOL_GAP: u32 = 10;
const TOOL_H: u32 = 32;

// The search field's width, and the window width below which it collapses to a
// bare magnifier so the breadcrumb keeps a usable share of a narrow header.
pub const SEARCH_W: u32 = 160;
pub const FIELD_MIN_W: u32 = 1000;
pub const CLEAR_W: u32 = 26;

// Icon-control metrics: the square box a bare icon button occupies, the glyph
// centred inside it, the gap from a glyph to the label beside it, and the inner
// padding of a pill that leads with an icon rather than with text.
const ICON_BTN: u32 = 30;
pub const GLYPH_S: u32 = 16;
const GLYPH_GAP: u32 = 6;
const ICON_PAD: u32 = 10;

// The side a close mark is drawn at. The toolkit's cross spans a little over
// half its box, so this side gives the eight-pixel cross the chrome carries.
pub const CROSS_S: u32 = 14;

// Breadcrumb metrics. The root is drawn as a home glyph of this width and the
// separators as chevrons centred in a gap of this width.
pub const CRUMB_PX: f32 = 17.0;
pub const CRUMB_PAD: u32 = 10;
pub const CRUMB_SEP_W: u32 = 14;
pub const HOME_W: u32 = 18;
pub const CRUMB_ROOT: &str = "Root";

pub const UNDO_LABEL: &str = "Undo";
pub const NEW_LABEL: &str = "New";
pub const SEARCH_HINT: &str = "Search";

/// The control strip at the display scale in force, in drawing pixels: the
/// label size, the gap between controls, the band, a bare icon button's box,
/// a hand-drawn glyph's side, the gap after it and a pill's inner padding.
/// `GLYPH_S` itself stays logical for the widgets that scale an icon size.
pub struct Strip {
    pub px: f32,
    pub gap: u32,
    pub h: u32,
    pub icon_btn: u32,
    pub glyph: u32,
    pub glyph_gap: u32,
    pub icon_pad: u32,
}

/// The strip at the display scale in force: each logical size read once.
pub fn strip() -> Strip {
    Strip {
        px: font(TOOL_PX),
        gap: px(TOOL_GAP),
        h: px(TOOL_H),
        icon_btn: px(ICON_BTN),
        glyph: px(GLYPH_S),
        glyph_gap: px(GLYPH_GAP),
        icon_pad: px(ICON_PAD),
    }
}

/// What a header click targets. `Crumb` carries the index of the breadcrumb
/// segment, counting the root as 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadHit {
    Search,
    SearchClear,
    ViewList,
    ViewGrid,
    Sort,
    Undo,
    New,
    NavBack,
    NavFwd,
    Crumb(usize),
}

/// One laid-out header control: the box it occupies and what clicking it means.
pub struct Slot {
    pub x: u32,
    pub w: u32,
    pub hit: HeadHit,
}
