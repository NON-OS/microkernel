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
use super::submission::Submission;

/// Set Features (admin opcode 09h).
pub const OPCODE_SET_FEATURES: u32 = 0x09;
/// Number of Queues: I/O submission and completion queues requested.
pub const FID_NUMBER_OF_QUEUES: u8 = 0x07;
/// Host Memory Buffer: host memory a DRAM-less controller keeps its tables in.
pub const FID_HOST_MEMORY_BUFFER: u8 = 0x0d;

impl Submission {
    /// SET FEATURES `fid` with command dwords 11 to 15. No data moves through
    /// PRP; the host memory buffer's descriptor list is named in 13 and 14.
    pub const fn set_features(cid: u16, fid: u8, dw: [u32; 5]) -> Self {
        Self {
            cdw0: OPCODE_SET_FEATURES | ((cid as u32) << 16),
            nsid: 0,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            prp1: 0,
            prp2: 0,
            cdw10: fid as u32,
            cdw11: dw[0],
            cdw12: dw[1],
            cdw13: dw[2],
            cdw14: dw[3],
            cdw15: dw[4],
        }
    }
}
