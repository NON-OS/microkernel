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

//! GICv2 register values that are pure arithmetic, proved on the host.

/// `GICC_IAR` and `GICD_SGIR` both keep the INTID in bits 9:0.
const INTID_MASK: u32 = 0x3FF;
/// 1020 to 1023 are special: 1022 and 1023 are spurious reads.
const FIRST_SPECIAL: u32 = 1020;
const SGI_LAST: u32 = 15;
/// `GICD_SGIR.TargetListFilter`: 0 sends to the CPU list, 1 to all others.
const FILTER_ALL_OTHERS: u32 = 1 << 24;
const TARGET_LIST_SHIFT: u32 = 16;

/// The INTID in a `GICC_IAR` value, `None` for a spurious or special read,
/// which must not be acknowledged back.
pub const fn iar_intid(raw: u32) -> Option<u32> {
    let intid = raw & INTID_MASK;
    if intid >= FIRST_SPECIAL {
        None
    } else {
        Some(intid)
    }
}

/// `GICD_SGIR` for SGI `intid` to the CPU interfaces in `targets`. Refuses a
/// non-SGI and an empty list, which the GIC would accept and deliver nowhere.
pub const fn sgir_to(targets: u8, intid: u32) -> Option<u32> {
    if intid > SGI_LAST || targets == 0 {
        return None;
    }
    Some(((targets as u32) << TARGET_LIST_SHIFT) | intid)
}

/// `GICD_SGIR` for SGI `intid` to every CPU but the sender.
pub const fn sgir_all_others(intid: u32) -> Option<u32> {
    if intid > SGI_LAST {
        return None;
    }
    Some(FILTER_ALL_OTHERS | intid)
}
