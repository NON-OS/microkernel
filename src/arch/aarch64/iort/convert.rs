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

//! One root complex node turned into its record.

use super::compose::routes;
use super::error::IortError;
use super::node::Node;
use super::records::RootComplex;
use super::route::handle;
use super::units::{first_its, segment, smmu};
use crate::arch::fdt::find::msi::{MsiController, MsiKind};

/// An ITS's register window: the control frame, then the translation frame.
const ITS_WINDOW: u64 = 0x2_0000;

pub(super) fn record(
    table: &[u8],
    node: &Node,
    its_base: &impl Fn(u32) -> Option<u64>,
) -> Result<RootComplex, IortError> {
    let found = routes(table, node)?;
    let smmu = match found.smmu {
        Some((unit, map)) => Some((smmu(&unit)?, map)),
        None => None,
    };
    let msi = match found.its {
        Some((group, map)) => {
            let base = its_base(first_its(&group)?).ok_or(IortError::UnknownIts)?;
            let size = ITS_WINDOW;
            let kind = MsiKind::Its;
            Some((MsiController { kind, base, size, phandle: handle(&group)? }, map))
        }
        None => None,
    };
    Ok(RootComplex { segment: segment(node)?, smmu, msi })
}
