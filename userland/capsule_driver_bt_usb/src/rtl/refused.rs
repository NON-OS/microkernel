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

//! Why a Realtek controller was not patched. The controller is then given
//! back unpatched; a ROM without its patch is not driven.

use nonos_bt_hci::HciError;

/// What in a pinned patch file did not read as this chip's patch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    Signature,
    Table,
    NoPatch,
    Bounds,
    Project,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// The controller refused a command, or did not answer it.
    Hci(HciError),
    /// No file by that name is built in.
    Missing(&'static str),
    /// The file is not the one pinned under that name.
    NotPinned(&'static str),
    /// The pinned file holds no patch for this controller.
    Layout(&'static str, Layout),
    /// Every fragment was taken, yet the controller still answers as its
    /// ROM: the patch is not running.
    NotTaken,
}

impl From<HciError> for Refused {
    fn from(e: HciError) -> Self {
        Self::Hci(e)
    }
}

impl Refused {
    /// The file it concerns, if any, and what was wrong.
    pub fn words(&self) -> (&'static str, &'static [u8]) {
        match self {
            Self::Hci(_) => ("", b"the controller refused the setup"),
            Self::Missing(f) => (f, b" is not built into this capsule"),
            Self::NotPinned(f) => (f, b" is not the pinned file: refused, nothing sent"),
            Self::Layout(f, Layout::NoPatch) => (f, b" has no patch for this ROM version"),
            Self::Layout(f, _) => (f, b" does not read as this chip's patch file"),
            Self::NotTaken => ("", b"the patch was sent but is not running"),
        }
    }
}
