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

//! The requester ids one IVHD device entry names (AMD IOMMU spec 48882,
//! 5.2.2.2, Linux amd/init.c init_iommu_from_acpi).

use super::ivhd_scope::Span;

/// What one entry adds: up to two spans. A range runs from a START (03h,
/// 43h, 47h) to the next END (04h); an alias entry (42h, 43h) also names the
/// id requests arrive under, at offset 5, as a special entry (48h) names an
/// IOAPIC or HPET.
pub(super) fn spans_of(
    kind: u8,
    id: u16,
    alias: u16,
    start: &mut Option<u16>,
) -> [Option<Span>; 2] {
    let one = |first, last, named| Some(Span { first, last, named });
    match kind {
        0x01 => [one(0, u16::MAX, false), None],
        0x02 | 0x46 | 0xF0 => [one(id, id, true), None],
        0x03 | 0x47 => {
            *start = Some(id);
            [None, None]
        }
        0x43 => {
            *start = Some(id);
            [one(alias, alias, true), None]
        }
        0x04 => [start.take().and_then(|first| one(first, id, false)), None],
        0x42 => [one(id, id, true), one(alias, alias, true)],
        0x48 => [one(alias, alias, true), None],
        _ => [None, None],
    }
}
