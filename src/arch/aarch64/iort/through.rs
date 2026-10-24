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

//! Requester ids carried through an SMMU to the ITS group behind it.

use super::compose::entry;
use super::error::IortError;
use super::layout::ITS_GROUP;
use super::mapping::{count, mapping, Mapping};
use super::node::{find, Node};
use super::route::{handle, one, push};
use super::units::own_msi_mapping;
use crate::arch::fdt::find::smmu::IommuMap;

/// Requester ids `rc` sends to the SMMU as stream ids, carried on through
/// each of the SMMU's mappings to an ITS group: the overlap of the two
/// ranges, shifted by both.
pub(super) fn through<'t>(
    table: &'t [u8],
    smmu: &Node<'t>,
    rc: &Mapping,
    dids: &mut IommuMap,
    its_handle: &mut Option<u32>,
    its: &mut Option<Node<'t>>,
) -> Result<(), IortError> {
    let own = own_msi_mapping(smmu)?;
    for k in (0..count(smmu)?).filter(|&k| Some(k) != own) {
        let m = mapping(smmu, k)?;
        let group = find(table, m.target)?;
        if m.single || group.kind != ITS_GROUP {
            return Err(IortError::BadMapping);
        }
        let low = u64::from(rc.output.max(m.input));
        let high = (u64::from(rc.output) + u64::from(rc.length))
            .min(u64::from(m.input) + u64::from(m.length));
        if low >= high {
            continue;
        }
        one(its_handle, handle(&group)?)?;
        let rid = u64::from(rc.input) + low - u64::from(rc.output);
        let did = u64::from(m.output) + low - u64::from(m.input);
        let narrow = |v: u64| u32::try_from(v).map_err(|_| IortError::BadMapping);
        push(dids, entry(narrow(rid)?, &group, narrow(did)?, narrow(high - low)?)?)?;
        *its = Some(group);
    }
    Ok(())
}
