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

//! The editor's menus, under the `editor` path their visibility names, each
//! the real editor source.

#[path = "../../../capsule_text_editor/src/editor/activity_bar.rs"]
pub mod activity_bar;
#[path = "../../../capsule_text_editor/src/editor/explorer.rs"]
pub mod explorer;
mod home;
#[path = "../../../capsule_text_editor/src/editor/manifest.rs"]
pub mod manifest;
#[path = "../../../capsule_text_editor/src/editor/menu_view.rs"]
pub mod menu_view;
pub mod menubar;
#[path = "../../../capsule_text_editor/src/editor/panel.rs"]
pub mod panel;
#[path = "../../../capsule_text_editor/src/editor/panel_view.rs"]
pub mod panel_view;
pub mod ribbon;
#[path = "../../../capsule_text_editor/src/editor/ribbon/square.rs"]
pub mod square;
#[path = "../../../capsule_text_editor/src/editor/screen.rs"]
pub mod screen;
mod settings;
#[path = "../../../capsule_text_editor/src/editor/shell.rs"]
pub mod shell;
#[path = "../../../capsule_text_editor/src/editor/specials.rs"]
pub mod specials;
#[path = "../../../capsule_text_editor/src/editor/strip.rs"]
pub mod strip;
#[path = "../../../capsule_text_editor/src/editor/strip_draw.rs"]
mod strip_draw;
pub use crate::layout;
pub use crate::state;
pub use crate::theme;
#[path = "../../../capsule_text_editor/src/editor/wordcount.rs"]
pub mod wordcount;

mod explorer_chevron_tests;
mod explorer_tests;
mod layout_scale_tests;
mod menu_lit_tests;
mod menu_rows_tests;
mod panel_count_tests;
mod panel_tests;
mod rail_tests;
mod square_look_tests;
pub(crate) mod square_tests;
mod status_tests;
mod strip_look_tests;
mod strip_tests;
