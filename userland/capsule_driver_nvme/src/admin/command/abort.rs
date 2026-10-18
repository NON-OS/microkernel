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

/// Abort (admin opcode 08h).
pub const OPCODE_ABORT: u32 = 0x08;

impl Submission {
    /// ABORT command `target` on submission queue `sqid`: CDW10 carries the
    /// command id in bits 31:16 and the queue in 15:0. Best effort: the
    /// controller may finish the command instead, and says which in DW0.
    pub const fn abort(cid: u16, sqid: u16, target: u16) -> Self {
        Self {
            cdw0: OPCODE_ABORT | ((cid as u32) << 16),
            nsid: 0,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            prp1: 0,
            prp2: 0,
            cdw10: ((target as u32) << 16) | sqid as u32,
            cdw11: 0,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}
