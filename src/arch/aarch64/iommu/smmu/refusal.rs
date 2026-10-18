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

use super::encode::idr::Unusable;

/// A unit the driver leaves off, said the way the boot log says it.
pub(super) fn reason(why: Unusable) -> &'static str {
    match why {
        Unusable::NoStage1 => "the SMMUv3 has no stage 1 translation",
        Unusable::NoAarch64Tables => "the SMMUv3 does not walk AArch64 tables",
        Unusable::NoTwoLevelStreamTable => "the SMMUv3 has no two-level stream table",
        Unusable::NotLittleEndian => "the SMMUv3 walks big-endian tables only",
        Unusable::NotCoherent => "the SMMUv3 cannot snoop the CPU caches",
        Unusable::No4kGranule => "the SMMUv3 has no 4 KiB granule",
    }
}
