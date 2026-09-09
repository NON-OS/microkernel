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

// Control-strip metrics: the em size every control's label is measured and drawn
// at, the pill's inner padding, the gap between controls, and the pill band.
pub const TOOL_PX: f32 = 17.0;
pub const TOOL_PAD: u32 = 14;
pub const TOOL_GAP: u32 = 10;
pub const TOOL_H: u32 = 32;
pub const SEARCH_W: u32 = 190;

// Breadcrumb metrics. The root segment is named rather than drawn as a bare
// slash, so the separators between segments read unambiguously.
pub const CRUMB_PX: f32 = 26.0;
pub const CRUMB_Y: u32 = 26;
pub const CRUMB_H: u32 = 34;
pub const CRUMB_ROOT: &str = "Root";
pub const CRUMB_SEP: &str = " / ";

pub const LIST_LABEL: &str = "List";
pub const GRID_LABEL: &str = "Grid";
pub const UNDO_LABEL: &str = "Undo";
pub const SEARCH_HINT: &str = "Search";

/// What a header click targets. `Crumb` carries the index of the breadcrumb
/// segment, counting the root as 0.
#[derive(Clone, Copy, PartialEq)]
pub enum HeadHit {
    Search,
    ViewList,
    ViewGrid,
    Sort,
    Undo,
    Crumb(usize),
}

/// One laid-out header control: the box it occupies and what clicking it means.
pub struct Slot {
    pub x: u32,
    pub w: u32,
    pub hit: HeadHit,
}
