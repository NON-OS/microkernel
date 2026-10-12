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

//! A value from HMAT's matrices: the one for this hierarchy level and data
//! type, the entry for this initiator and target domain, times the base
//! unit. An entry of 0 means firmware gave no value; 0xFFFF means the
//! target cannot be reached from the initiator.

use super::hmat_entry::Matrix;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HmatValue {
    Value(u64),
    Unreachable,
}

pub fn query(
    matrices: &[Matrix],
    hierarchy: u8,
    data_type: u8,
    initiator: u32,
    target: u32,
) -> Option<HmatValue> {
    let m = matrices.iter().find(|m| (m.hierarchy, m.data_type) == (hierarchy, data_type))?;
    let i = m.initiators.iter().position(|d| *d == initiator)?;
    let t = m.targets.iter().position(|d| *d == target)?;
    match *m.entries.get(i * m.targets.len() + t)? {
        0 => None,
        0xFFFF => Some(HmatValue::Unreachable),
        e => u64::from(e).checked_mul(m.base_unit).map(HmatValue::Value),
    }
}
