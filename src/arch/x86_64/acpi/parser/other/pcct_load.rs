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

//! PCCT read once at boot through the one parser (`shared_tables::pcct`)
//! and logged: the communication channels CPPC and RAS use on servers.
//! Nothing in NONOS rings a doorbell or writes a channel's shared memory.

use super::super::state::TableRegistry;
use super::raw_table::bytes;
use super::shared_tables::pcct::pcct;

pub fn parse_pcct(registry: &mut TableRegistry) {
    let Some(p) = bytes(registry, b"PCCT").and_then(|t| pcct(t).ok()) else { return };
    let known = p.subspaces.iter().flatten().count();
    crate::log::info!("[ACPI] PCCT: {} subspaces, {} read", p.subspaces.len(), known);
}
