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

#[derive(Debug, Clone, Copy)]
pub struct IoApicInfo {
    pub id: u8,
    pub address: u64,
    pub gsi_base: u32,
}

impl IoApicInfo {
    /* gsi_base comes from the MADT unchecked; saturate rather than abort. */
    pub fn gsi_max(&self) -> u32 {
        self.gsi_base.saturating_add(23)
    }
}

/// The IOAPIC whose input range holds `gsi`: the one with the highest GSI
/// base at or below it. The MADT does not say how many inputs an IOAPIC
/// has (that is in its version register), and assuming 24 loses every GSI
/// above 23 on chipsets with more pins: Gemini Lake's IOAPIC has 120.
/// Ranges of distinct IOAPICs never overlap (ACPI 6.5 section 5.2.13), so the
/// nearest base below is the owner.
pub fn owner_of_gsi(ioapics: &[IoApicInfo], gsi: u32) -> Option<&IoApicInfo> {
    ioapics.iter().filter(|io| io.gsi_base <= gsi).max_by_key(|io| io.gsi_base)
}
