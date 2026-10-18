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

//! MCFG read once at boot through the one pure decoder (`mcfg_decode`),
//! which never reads past the bytes the header's length bounds.

#[path = "mcfg_decode.rs"]
pub(super) mod mcfg_decode;

use super::super::state::TableRegistry;
use super::raw_table::bytes;
use crate::arch::x86_64::acpi::data::PcieSegment;

pub fn parse_mcfg(registry: &mut TableRegistry) {
    let Some(table) = bytes(registry, b"MCFG") else { return };
    for w in mcfg_decode::windows(table) {
        registry.data.pcie_segments.push(PcieSegment {
            base_address: w.base,
            segment: w.segment,
            start_bus: w.start_bus,
            end_bus: w.end_bus,
        });
    }
}
