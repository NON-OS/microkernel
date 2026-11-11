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

//! The ribbon's painter, its cell walk, metrics and hit test, each the real
//! editor source.

#[path = "../../../../capsule_text_editor/src/editor/ribbon/cell.rs"]
pub mod cell;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/cells.rs"]
pub mod cells;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/hit.rs"]
pub mod hit;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/items.rs"]
pub mod items;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/metrics.rs"]
pub mod metrics;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/paint.rs"]
pub mod paint;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/panel.rs"]
pub mod panel;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/snapshot.rs"]
pub mod snapshot;
pub use super::square;
#[path = "../../../../capsule_text_editor/src/editor/ribbon/squares.rs"]
pub mod squares;

mod paint_tests;
mod scale_tests;
