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
//! What UAS needs of a SCSI command, and nothing more: its CDB, which way
//! its data moves and how much, and where its status and sense go. The
//! SCSI crate the caller runs on fits this in one file.

/// Which way a command's data moves, if at all.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DataDir {
    None,
    /// Device to host, on the data-in pipe.
    In,
    /// Host to device, on the data-out pipe.
    Out,
}

pub trait ScsiCommand {
    /// The CDB: 6 to 16 bytes (longer ones would need additional CDB
    /// bytes, which this does not send).
    fn cdb(&self) -> &[u8];
    fn dir(&self) -> DataDir;
    /// Bytes of data the command moves, 0 for none.
    fn length(&self) -> u32;
    /// The command ended with SCSI `status` and, when the device gave it,
    /// `sense` data.
    fn finished(&mut self, status: u8, sense: &[u8]);
}
