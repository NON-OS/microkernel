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

//! Where review puts the answers, by the mode chosen: kept in the store of
//! the disk this boot came from, or held in memory for the installer. No
//! crate paths here, so the host proofs mount this file as it is.

/// The Mode step's rows after the first, which is amnesic.
pub const USB_LIVE: u8 = 1;
pub const INSTALL: u8 = 2;

/// What review does with the answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answers {
    /// Persisted, the marker after them, in the store of the disk this boot
    /// came from, so the next boot from that disk skips setup.
    Save,
    /// Written to the vfs in memory alone. The installer reads them there
    /// and carries them, with a marker, to the disk it writes.
    Stage,
}

/*
 * Only a mode that keeps state on the disk this boot came from saves there:
 * USB live, which the Mode step lists but does not let ENTER take today.
 * Install keeps state on the disk the installer writes, not on this one.
 * Saved here, the answers made every later boot from the stick skip setup
 * and come up as the installed system, with nothing amnesic about it.
 */
pub fn answers_for(mode: u8) -> Answers {
    if mode == USB_LIVE {
        Answers::Save
    } else {
        Answers::Stage
    }
}
