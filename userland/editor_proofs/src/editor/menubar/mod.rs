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

//! The menu bar's row tables, the items built from them, its measured title
//! spans and its hit test.

#[path = "../../../../capsule_text_editor/src/editor/menubar/items.rs"]
pub mod items;
#[path = "../../../../capsule_text_editor/src/editor/menubar/tables.rs"]
pub mod tables;
#[path = "../../../../capsule_text_editor/src/editor/menubar/hit.rs"]
pub mod hit;
#[path = "../../../../capsule_text_editor/src/editor/menubar/metrics.rs"]
pub mod metrics;

mod scale_tests;
