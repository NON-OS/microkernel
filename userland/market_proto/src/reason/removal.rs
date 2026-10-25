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

//! Codes only an uninstall stops with.

use super::{said, Reason};

/// An uninstall found no install of the package recorded on this system.
pub const WHY_NOT_INSTALLED: u8 = 22;
/// An uninstall of a shipped tier on a system built without the model
/// fetcher, which is what takes a model off the data volume.
pub const WHY_MODEL_KEPT: u8 = 23;
/// An uninstall that left files or links behind, still recorded.
pub const WHY_REMOVE_PARTIAL: u8 = 24;
/// An uninstall of a tier part of whose model is still downloading.
pub const WHY_MODEL_DOWNLOADING: u8 = 26;

/// Whether `code` is one only an uninstall stops with, so a client says to
/// uninstall again rather than install.
pub fn removal_only(code: u8) -> bool {
    matches!(code, WHY_NOT_INSTALLED | WHY_MODEL_KEPT | WHY_REMOVE_PARTIAL | WHY_MODEL_DOWNLOADING)
}

pub(super) fn removal_reason(code: u8) -> Option<Reason> {
    Some(match code {
        WHY_NOT_INSTALLED => said("Nothing of it is installed on this system", false),
        WHY_MODEL_KEPT => said(
            "This system was built without the model fetcher, which takes a model off the \
             data volume, so its model stays",
            false,
        ),
        WHY_REMOVE_PARTIAL => {
            said("Some of its files would not go; uninstall again to finish", true)
        }
        WHY_MODEL_DOWNLOADING => said(
            "Part of its model is still downloading; let it finish or stop it, \
             then uninstall again",
            true,
        ),
        _ => return None,
    })
}
