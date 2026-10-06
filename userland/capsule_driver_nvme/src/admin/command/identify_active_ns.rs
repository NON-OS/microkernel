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

impl Submission {
    /// Identify with CNS 02h: the active namespace list, up to 1024 NSIDs
    /// above `nsid` in ascending order, zero padded. NVMe 1.1 added it; a
    /// 1.0 controller fails it as an invalid field.
    pub const fn identify_active_ns_list(cid: u16, nsid: u32, prp1: u64) -> Self {
        Self {
            cdw0: 0x06 | ((cid as u32) << 16),
            nsid,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            prp1,
            prp2: 0,
            cdw10: 2,
            cdw11: 0,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}
