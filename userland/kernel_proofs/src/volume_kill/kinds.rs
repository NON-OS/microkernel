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

//! The shape of the directory a scenario starts from, and one request to
//! the device.

use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Dir {
    /// R0 has room for another entry.
    HasRoom,
    /// R0 is full: a create appends a record.
    Full,
    /// The directory has no record yet: a create makes its first.
    NoRecord,
}

/// One request to the device: a header, or a whole block.
#[derive(Clone, Debug)]
pub(super) enum Write {
    Header { generation: u64, free: u64 },
    Block { lba: u64, to: BTreeSet<u64>, rec: bool },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Op {
    Create,
    Unlink,
    Close,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Mount {
    pub(super) generation: u64,
    pub(super) free: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Knob {
    Fixed,
    /// H works on a copy of the mount, gone when it is swept.
    LocalCopy,
    /// H's request in flight lands after C's, not before.
    NoFifo,
}

/// What became of H's request in flight.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Fate {
    Lands,
    Never,
}
