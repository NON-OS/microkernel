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

//! What reading a disk's GPT came to.

use alloc::vec::Vec;

use super::entry::Entry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Held {
    Primary,
    /// The primary failed; the backup is the table.
    Backup,
    /// Both hold but disagree on the disk, its usable range or its array;
    /// the primary is the table, and the log says so.
    Disagreeing,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Table {
    /// No protective MBR and no GPT signature at LBA 1.
    NoGpt,
    /// A GPT shown, by its MBR or its signature, with no table to believe.
    Unbelieved,
    Named(Vec<Entry>, Held),
}
