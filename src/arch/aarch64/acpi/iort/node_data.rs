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

//! What each node type carries after the 16-byte header (Arm DEN0049;
//! offsets from the node's start). Each reader is handed the whole node,
//! already inside the table, and answers None if the node is too short.

use super::super::le::{u32_at, u64_at, u8_at};
use super::node::{Kind, ITS_GROUP, NAMED_COMPONENT, RMR, ROOT_COMPLEX, SMMU_V1_V2, SMMU_V3};
use super::node_smmu::{rmr, smmu_v2};
use alloc::vec::Vec;

fn its_group(n: &[u8]) -> Option<Kind> {
    let count = u32_at(n, 16)? as usize;
    let ids = (0..count).map(|i| u32_at(n, 20usize.checked_add(i.checked_mul(4)?)?));
    Some(Kind::ItsGroup { its_ids: ids.collect::<Option<Vec<_>>>()? })
}

/// The device object's name, NUL-terminated inside the node.
fn named(n: &[u8]) -> Option<Kind> {
    let tail = n.get(29..)?;
    let end = tail.iter().position(|c| *c == 0)?;
    let (flags, cca, address_limit) = (u32_at(n, 16)?, u32_at(n, 20)?, u8_at(n, 28)?);
    Some(Kind::NamedComponent { flags, cca, address_limit, name: tail[..end].to_vec() })
}

fn root_complex(n: &[u8], revision: u8) -> Option<Kind> {
    let address_limit = if revision >= 1 { u8_at(n, 32) } else { None };
    Some(Kind::RootComplex {
        cca: u32_at(n, 16)?,
        ats: u32_at(n, 24)?,
        segment: u32_at(n, 28)?,
        address_limit,
    })
}

fn smmu_v3(n: &[u8]) -> Option<Kind> {
    let flags = u32_at(n, 24)?;
    Some(Kind::Smmuv3 {
        base: u64_at(n, 16)?,
        flags,
        model: u32_at(n, 40)?,
        gsivs: [u32_at(n, 44)?, u32_at(n, 48)?, u32_at(n, 52)?, u32_at(n, 56)?],
        proximity: u32_at(n, 60).filter(|_| flags & (1 << 3) != 0),
        id_mapping_index: u32_at(n, 64).filter(|_| flags & (1 << 4) != 0),
    })
}

pub(crate) fn kind(n: &[u8], ty: u8, revision: u8) -> Option<Kind> {
    match ty {
        ITS_GROUP => its_group(n),
        NAMED_COMPONENT => named(n),
        ROOT_COMPLEX => root_complex(n, revision),
        SMMU_V1_V2 => smmu_v2(n),
        SMMU_V3 => smmu_v3(n),
        RMR => rmr(n),
        other => Some(Kind::Other(other)),
    }
}
