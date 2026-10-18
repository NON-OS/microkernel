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

//! What the kernel's idle code asks of the low-power idle levels the ACPI
//! capsule handed over (ACPI 6.5, 8.4.4): a container's state may be
//! entered only when every processor in it has asked for it, so the code
//! needs who shares a container. Read from `cpu_store`; nothing here
//! enters a state.

use super::cpu_store::{tables, uids};
use super::lpi_record::LpiLevel;
use alloc::vec::Vec;

/// The processor's levels, its own first; empty when it idles by _CST or
/// the capsule has not handed its tables over.
pub fn levels(uid: u64) -> Vec<LpiLevel> {
    tables(uid).map(|t| t.levels).unwrap_or_default()
}

/// The processors, by ACPI UID, whose hierarchy holds the container with
/// this _UID at this level (1 is the nearest container).
pub fn sharing(level: usize, container: u64) -> Vec<u64> {
    let holds = |uid: &u64| {
        tables(*uid).is_some_and(|t| {
            t.levels.get(level).is_some_and(|l| l.container == Some(container))
        })
    };
    uids().into_iter().filter(holds).collect()
}
