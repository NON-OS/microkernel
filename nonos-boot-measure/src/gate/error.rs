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

use crate::authenticode::PeError;
use crate::record::RecordError;
use crate::tcg::LogError;

/// Why the bootloader was not admitted. `code` is stable and grouped: 1 to 9
/// the chain, 100s the log, 200s the record, 300s the image, 400s the STARK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    /// The TCG log does not replay to the PCR 4 the TPM holds.
    PcrMismatch,
    /// The log names no application in PCR 4.
    NoApplication,
    /// The trailer is not a v4 bootloader trailer.
    Trailer,
    /// The measurement's path does not fold to the signed root.
    Path,
    /// The signed root is not four canonical words.
    Root,
    Log(LogError),
    Record(RecordError),
    Image(PeError),
    /// `nox_verify` refused the proof, with its own code.
    Proof(u32),
}

impl BootError {
    pub const fn code(self) -> u32 {
        match self {
            BootError::PcrMismatch => 1,
            BootError::NoApplication => 2,
            BootError::Trailer => 3,
            BootError::Path => 4,
            BootError::Root => 5,
            BootError::Log(e) => 100 + e.code(),
            BootError::Record(e) => 200 + e.code(),
            BootError::Image(e) => 300 + e.code(),
            BootError::Proof(c) => 400 + c,
        }
    }
}
