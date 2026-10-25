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

//! One root complex's maps: requester id to stream id at its SMMU, and
//! requester id to device id at its ITS group, directly or through the SMMU.

use super::error::IortError;
use super::layout::{ITS_GROUP, SMMU_V3};
use super::mapping::{count, mapping};
use super::node::{find, Node};
use super::route::{handle, one, push, EMPTY};
use super::through::through;
use crate::arch::fdt::find::smmu::{IommuMap, MapEntry};

/// Each map with the node it sends to, `None` where nothing is sent.
pub(super) struct Routes<'t> {
    pub smmu: Option<(Node<'t>, IommuMap)>,
    pub its: Option<(Node<'t>, IommuMap)>,
}

pub(super) fn entry(
    rid_base: u32,
    unit: &Node,
    id_base: u32,
    length: u32,
) -> Result<MapEntry, IortError> {
    Ok(MapEntry { rid_base, phandle: handle(unit)?, sid_base: id_base, length })
}

pub(super) fn routes<'t>(table: &'t [u8], rc: &Node<'t>) -> Result<Routes<'t>, IortError> {
    let (mut sids, mut dids) = (EMPTY, EMPTY);
    let (mut smmu, mut its) = (None, None);
    let (mut smmu_handle, mut its_handle) = (None, None);
    for n in 0..count(rc)? {
        let m = mapping(rc, n)?;
        // A single mapping is for a device with no requester id; every
        // function behind a root complex has one.
        if m.single {
            return Err(IortError::BadMapping);
        }
        let unit = find(table, m.target)?;
        match unit.kind {
            SMMU_V3 => {
                one(&mut smmu_handle, handle(&unit)?)?;
                push(&mut sids, entry(m.input, &unit, m.output, m.length)?)?;
                through(table, &unit, &m, &mut dids, &mut its_handle, &mut its)?;
                smmu = Some(unit);
            }
            ITS_GROUP => {
                one(&mut its_handle, handle(&unit)?)?;
                push(&mut dids, entry(m.input, &unit, m.output, m.length)?)?;
                its = Some(unit);
            }
            _ => return Err(IortError::BadMapping),
        }
    }
    Ok(Routes { smmu: smmu.map(|s| (s, sids)), its: its.map(|g| (g, dids)) })
}
