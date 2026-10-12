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

use crate::identity::Refusal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AhciError {
    DeviceNotFound,
    /// No ATA disk on a port: its link stayed quiet after COMRESET (nothing
    /// attached), or the device on it is no ATA disk (an optical drive). On
    /// the served port, before issue: PxSSTS reads no device or the PHY
    /// offline (the disk pulled), or a device whose link is down inside the
    /// window after the COMRESET sent to bring it back.
    NoDisk,
    BrokerCallFailed(i64),
    /// The drive ended the issued command with an error of its own (TFES,
    /// ERR in PxTFD).
    CommandFailed,
    /// The HBA failed the issued command with an error of its own (HBFS,
    /// HBDS, IFS, OFS), the link changed under it (PRCS, PCS), or the port
    /// shows slots the driver never issued, during a command or still after
    /// recovery before one: PxTFD says nothing about it, so the disk is not
    /// blamed.
    HbaFailed,
    Timeout,
    /// The disk's IDENTIFY block breaks the named rule in `identity`.
    IdentityRefused(Refusal),
    /// A transfer named sectors outside the served disk, or more bytes than
    /// the data buffer holds; no command was built.
    OutOfRange,
    /// A read or a write ended with a clean status but the HBA counts another
    /// number of bytes moved than it asked for (PRDBC). A short read leaves
    /// an earlier request's data in the buffer's tail, a short write did not
    /// reach the disk whole; long, the count is not to be believed.
    ShortTransfer,
    /// The port's command engine would not stop, even after a COMRESET, so
    /// the HBA may still own its command list and buffer: nothing more is
    /// built or issued on it.
    Wedged,
}

pub type AhciResult<T> = Result<T, AhciError>;
