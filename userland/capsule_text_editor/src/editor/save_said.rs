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

//! What the status bar says when a save or an export did not land.
//!
//! Every failed write read "save failed: file could not be written" (and an
//! export just "export failed"), whether the store was full, did not answer,
//! or had no folder by that name, so a person could not tell what to change.
//! The vfs client's reason is said in words, with what to do where there is
//! something.

/// The status line for a save that failed with the vfs client's `err`.
pub fn save_failed(err: &str) -> &'static [u8] {
    match err {
        "vfs ipc failed" => b"save failed: the file store did not answer; try again",
        "no space left" => b"save failed: the file store is full; delete files to make room",
        "too large" => b"save failed: the document is too large for the file store",
        "access denied" => b"save failed: this place cannot be written; save somewhere else",
        "is a directory" => b"save failed: a folder has that name; choose another name",
        "vfs open failed" | "not found" => {
            b"save failed: the file could not be made there; check the folder exists"
        }
        "vfs path invalid" => b"save failed: that path is too long",
        _ => b"save failed: file could not be written",
    }
}

/// The status line for an export that failed with `err`: the save's words,
/// said of the export.
pub fn export_failed(err: &str) -> &'static [u8] {
    match err {
        "vfs ipc failed" => b"export failed: the file store did not answer; try again",
        "no space left" => b"export failed: the file store is full; delete files to make room",
        "too large" => b"export failed: the export is too large for the file store",
        "access denied" => b"export failed: this place cannot be written; export elsewhere",
        "is a directory" => b"export failed: a folder has that name; choose another name",
        "vfs open failed" | "not found" => {
            b"export failed: the file could not be made there; check the folder exists"
        }
        "vfs path invalid" => b"export failed: that path is too long",
        _ => b"export failed: file could not be written",
    }
}
