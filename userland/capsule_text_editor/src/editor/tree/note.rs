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

//! Where and how the explorer says its last failure. `FileTree::status` holds
//! the vfs client's reason a listing, create, rename or delete failed; it used
//! to be drawn only in place of an empty tree, so a failed delete or rename in
//! a tree with rows changed nothing on screen.

/// The explorer's line for the failure behind `status`, short enough for its
/// header band; a reason it does not know is shown as the client worded it.
pub fn explorer_note(status: &'static str) -> Option<&'static str> {
    match status {
        "" => None,
        "vfs ipc failed" => Some("store did not answer"),
        "vfs list failed" => Some("store would not list files"),
        "vfs path invalid" => Some("path is too long"),
        "vfs open failed" => Some("could not be made there"),
        other => Some(other),
    }
}

/// What an empty tree's single row says: the failure when there was one, else
/// that the store holds nothing.
pub fn empty_tree_line(status: &'static str) -> &'static str {
    explorer_note(status).unwrap_or("(empty)")
}
