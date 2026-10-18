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

//! PCCT, the platform communication channel table (ACPI 6.5, 14.1;
//! recalled, unverified): flags u32 at 36 (bit 0, the platform signals
//! completion by interrupt), eight reserved bytes, then the subspaces from
//! 48, each a type u8 and a length u8. A subspace's identifier is its
//! place in the table, so one this parser cannot read keeps its place as
//! None. Read-only: nothing here, or anywhere in NONOS, rings a doorbell.

use super::le::u32_at;
use super::pcct_subspace::subspace;
pub use super::pcct_types::{Reg, Subspace};
use super::sdt::{table, TableError};
use alloc::vec::Vec;

const SUBSPACES_AT: usize = 48;
/// Subspace identifiers are a byte in the PCC address space.
pub const MAX_SUBSPACES: usize = 256;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pcct {
    pub interrupt_completion: bool,
    pub subspaces: Vec<Option<Subspace>>,
}

pub fn pcct(t: &[u8]) -> Result<Pcct, TableError> {
    let t = table(t, b"PCCT")?;
    let flags = u32_at(t, 36).ok_or(TableError::Short)?;
    let mut out = Pcct { interrupt_completion: flags & 1 != 0, subspaces: Vec::new() };
    let mut at = SUBSPACES_AT;
    while at < t.len() && out.subspaces.len() < MAX_SUBSPACES {
        let len = usize::from(*t.get(at + 1).ok_or(TableError::Structure(at))?);
        let s = t.get(at..at + len).filter(|_| len >= 2).ok_or(TableError::Structure(at))?;
        out.subspaces.push(subspace(s));
        at += len;
    }
    Ok(out)
}
