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

/// Why the driver will not serve a disk, on what its IDENTIFY block says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Words 83 and 86, read under their validity bits, do not both say the
    /// 48-bit Address feature set is supported and enabled.
    No48BitAddress,
    /// Word 106 and words 117-118 give a logical sector other than 512 bytes.
    SectorSize,
    /// Words 100-103 count no sectors.
    NoCapacity,
    /// Words 100-103 count 2^48 sectors or more, past what a 48-bit LBA names.
    PastLba48,
}
