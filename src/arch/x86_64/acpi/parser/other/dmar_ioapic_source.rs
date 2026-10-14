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

//! An IO-APIC scope's requester id. Pure; kernel_proofs::dmar_scope.

use super::dmar_ioapic::IoapicScope;
use super::dmar_scope::MAX_PATH;

/// The requester id: the last hop's device and function on the bus the path
/// reaches, each bridge before it read for its secondary bus by `secondary`
/// (as Linux's ir_parse_one_ioapic_scope).
pub fn source_id(s: &IoapicScope, secondary: impl Fn(u8, u8, u8) -> Option<u8>) -> Option<u16> {
    let n = s.path_len as usize;
    if n == 0 || n > MAX_PATH || s.path[..n].iter().any(|&(d, f)| d > 31 || f > 7) {
        return None;
    }
    let mut bus = s.start_bus;
    for &(d, f) in &s.path[..n - 1] {
        bus = secondary(bus, d, f)?;
    }
    let (d, f) = s.path[n - 1];
    Some((bus as u16) << 8 | (d as u16) << 3 | f as u16)
}
