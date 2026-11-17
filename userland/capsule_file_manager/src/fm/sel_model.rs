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

use nonos_toolkit::icons::IconId;

/// One action the selection band offers.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelAction {
    Move,
    Duplicate,
    Tag,
    Delete,
    Clear,
}

/// The band's one table: the glyph and the label both the painter and the
/// hit-test measure from. Every action here has a handler. There is no Share
/// or Compress: the capsule holds no sharing channel and the vfs client no
/// archive op, so those would be buttons that do nothing.
pub const ACTIONS: [(SelAction, IconId, &str); 5] = [
    (SelAction::Move, IconId::Forward, "Move"),
    (SelAction::Duplicate, IconId::FsFile, "Duplicate"),
    (SelAction::Tag, IconId::Tag, "Tag"),
    (SelAction::Delete, IconId::Trash, "Delete"),
    (SelAction::Clear, IconId::Back, "Clear"),
];
