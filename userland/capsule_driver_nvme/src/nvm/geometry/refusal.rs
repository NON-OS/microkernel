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

//! Why a namespace gets no I/O queue.

/// Why a namespace gets no I/O queue, carrying what the console line names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The controller reported no namespace, or none is active.
    NoNamespace,
    /// NSZE is zero: an inactive NSID, or an empty namespace.
    Empty,
    /// A block size the buffers were not built for (LBADS of the format).
    BlockSize(u32),
    /// FLBAS names a format the namespace does not have, or one past the
    /// 16 slots the parser read. Carries FLBAS bits 3:0, bits 6:5 and NLBAF
    /// (zero based).
    Format { index: u8, upper: u8, nlbaf: u8 },
    /// The format carries metadata (MS bytes per block).
    Metadata(u16),
    /// MDTS lets one command move less than one block.
    TransferBelowBlock { max_bytes: u64, lba_size: u32 },
}
