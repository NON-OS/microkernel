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

//! Where a disk with no NONOS partitions keeps its store and plan: the fixed
//! sectors `userland/nonos_disk_map` names, which every NONOS disk used
//! before its table named them and which a stick and a QEMU data disk still
//! use. kernel_proofs holds these to that crate.

use super::places::{Extent, Places};

const STORE_BASE_LBA: u64 = 256;
const PLAN_LBA: u64 = 245_760;
const DATA_FLOOR: u64 = 262_144;

pub(crate) const FIXED: Places = Places {
    store: Extent { first: STORE_BASE_LBA, sectors: PLAN_LBA - STORE_BASE_LBA },
    plan: Extent { first: PLAN_LBA, sectors: DATA_FLOOR - PLAN_LBA },
    data: None,
};
