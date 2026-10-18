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

//! Every root complex IORT describes, as the records the device-tree path
//! fills for the one host bridge a tree names: the SMMU and its map, the
//! MSI controller and its map. The broker reads these and nothing else, so
//! a device behind an ACPI root complex binds the way a tree's does.

use super::convert::record;
use super::error::IortError;
use super::layout::ROOT_COMPLEX;
use super::node::each;
use super::read::{bytes, index};
use crate::arch::fdt::find::msi::MsiController;
use crate::arch::fdt::find::smmu::{IommuMap, SmmuInfo};

/// Root complexes kept; more is refused, not truncated.
pub const MAX_ROOT_COMPLEXES: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootComplex {
    pub segment: u32,
    pub smmu: Option<(SmmuInfo, IommuMap)>,
    pub msi: Option<(MsiController, IommuMap)>,
}

/// Every root complex in `table`, a whole IORT. `its_base` answers an ITS
/// identifier with its register base, which MADT's GIC ITS structures give;
/// this table only names ITSs by identifier.
pub fn root_complexes(
    table: &[u8],
    its_base: impl Fn(u32) -> Option<u64>,
) -> Result<[Option<RootComplex>; MAX_ROOT_COMPLEXES], IortError> {
    if bytes::<4>(table, 0)? != *b"IORT" {
        return Err(IortError::BadNode);
    }
    let table = table.get(..index(table, 4)?).ok_or(IortError::Short)?;
    let mut out = [None; MAX_ROOT_COMPLEXES];
    let mut kept = 0;
    each(table, |node| {
        if node.kind != ROOT_COMPLEX {
            return Ok(());
        }
        let slot = out.get_mut(kept).ok_or(IortError::TooMany)?;
        *slot = Some(record(table, &node, &its_base)?);
        kept += 1;
        Ok(())
    })?;
    Ok(out)
}
