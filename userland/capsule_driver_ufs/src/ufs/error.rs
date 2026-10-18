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

//! How a step fails, and the status a wire request gets for it.

/// A failure, each carrying what the host or the device said.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UfsError {
    /// A wait passed its bound; the step that waited logs which.
    Timeout,
    /// HCE would not read back, or the lists never came ready.
    Host,
    /// A UIC command answered this result (UCMDARG2).
    Uic(u8),
    /// Link startup finished with no device on the link (HCS.DP clear).
    NoDevice,
    /// The host ended a request with this Overall Command Status.
    Ocs(u8),
    /// The device answered with this transaction type, or a failure.
    Upiu(u8),
    /// A query answered this response code.
    Query(u8),
    /// A SCSI command ended CHECK CONDITION with this sense.
    Sense { key: u8, asc: u8, ascq: u8 },
    /// A SCSI command ended with this status, neither GOOD nor CHECK CONDITION.
    Status(u8),
    /// A DMA region above 4 GiB, and a host without 64-bit addressing.
    Dma32,
    /// A logical unit whose blocks are not 512 or 4096 bytes.
    BlockSize(u32),
    /// A logical unit of more blocks than its bytes can count.
    Capacity(u64),
}

/// DATA PROTECT and ILLEGAL REQUEST, the sense keys a caller can act on.
const KEY_ILLEGAL_REQUEST: u8 = 0x5;
const KEY_DATA_PROTECT: u8 = 0x7;

/// The wire status: -110 for a timeout, -30 for a unit that refuses writes,
/// -22 for a request the unit calls illegal, -5 for the rest.
pub fn wire_status(e: UfsError) -> i32 {
    match e {
        UfsError::Timeout => -110,
        UfsError::Sense { key: KEY_DATA_PROTECT, .. } => -30,
        UfsError::Sense { key: KEY_ILLEGAL_REQUEST, .. } => -22,
        _ => -5,
    }
}
