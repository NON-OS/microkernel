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

//! Row tables. The row index is the action selector, so order is load-bearing.
//! A shortcut names the keys that do the same as the row.

use nonos_toolkit::widgets::Item;

use super::items::MenuCmd::{
    CloseTab, Ctrl, Info, Markup, NewTab, PageBreak, Special, Table, Theme, ToggleSidebar,
    WordCount,
};
use super::items::{MenuCmd, MenuRow};

const fn row(label: &'static str, keys: &'static str, cmd: MenuCmd) -> MenuRow {
    (Item::new(label).shortcut(keys), cmd)
}

pub(super) const FILE: [MenuRow; 6] = [
    row("New Tab", "Ctrl+N", NewTab),
    row("Open...", "Ctrl+O", Ctrl(0x4F, false)),
    row("Save", "Ctrl+S", Ctrl(0x53, false)),
    row("Save As...", "Ctrl+Shift+S", Ctrl(0x53, true)),
    row("Export...", "Ctrl+E", Ctrl(0x45, false)),
    row("Close Tab", "Ctrl+W", CloseTab),
];
pub(super) const EDIT: [MenuRow; 9] = [
    row("Undo", "Ctrl+Z", Ctrl(0x5A, false)),
    row("Redo", "Ctrl+Y", Ctrl(0x59, false)),
    row("Cut", "Ctrl+X", Ctrl(0x58, false)),
    row("Copy", "Ctrl+C", Ctrl(0x43, false)),
    row("Paste", "Ctrl+V", Ctrl(0x56, false)),
    row("Select All", "Ctrl+A", Ctrl(0x41, false)),
    row("Find...", "Ctrl+F", Ctrl(0x46, false)),
    row("Replace...", "Ctrl+H", Ctrl(0x48, false)),
    row("Replace All", "Ctrl+Shift+H", Ctrl(0x48, true)),
];
pub(super) const VIEW: [MenuRow; 5] = [
    row("Zoom In", "Ctrl+=", Ctrl(0x3D, false)),
    row("Zoom Out", "Ctrl+-", Ctrl(0x2D, false)),
    row("Reset Zoom", "Ctrl+0", Ctrl(0x30, false)),
    // Not a Ctrl chord: Ctrl+B is the file tree, and the theme is cycled by
    // the two-key Ctrl+K T, which a single Ctrl row cannot send.
    row("Toggle Theme", "Ctrl+K T", Theme),
    row("Toggle Sidebar", "Ctrl+B", ToggleSidebar),
];
pub(super) const INSERT: [MenuRow; 4] = [
    row("Image", "", Markup(true)),
    row("Link", "", Markup(false)),
    row("Page Break", "", PageBreak),
    row("Special Character", "", Special),
];
pub(super) const FORMAT: [MenuRow; 4] = [
    row("Bold", "Ctrl+Shift+B", Ctrl(0x42, true)),
    row("Italic", "Ctrl+I", Ctrl(0x49, false)),
    row("Underline", "Ctrl+U", Ctrl(0x55, false)),
    row("Toggle Comment", "Ctrl+/", Ctrl(0x2F, false)),
];
pub(super) const TOOLS: [MenuRow; 3] = [
    row("Duplicate Line", "Ctrl+D", Ctrl(0x44, false)),
    (Item::new("Delete Line").shortcut("Ctrl+Shift+K").danger(), Ctrl(0x4B, true)),
    row("Word Count", "", WordCount),
];
pub(super) const TABLE: [MenuRow; 4] = [
    row("Insert Table", "", Table(0)),
    row("Insert Row", "", Table(1)),
    row("Insert Column", "", Table(2)),
    (Item::new("Delete Table").danger(), Table(3)),
];
pub(super) const HELP: [MenuRow; 2] =
    [row("Keyboard Shortcuts", "", Info(0)), row("About NONOS Docs", "", Info(1))];
