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

//! IORT: each node type read, IDs followed from a root complex through the
//! SMMU to the ITS, and mappings that name nothing, overrun their node or
//! loop.

use super::iort_build::*;
use crate::arm_acpi::iort::{iort, translate, Kind};

#[test]
fn every_node_and_the_way_through() {
    let n = iort(&iort_of(&nodes())).unwrap();
    assert_eq!(n.iter().map(|x| x.offset).collect::<Vec<_>>(), [ITS, SMMU, RC, NAMED]);
    assert_eq!(n[0].kind, Kind::ItsGroup { its_ids: vec![7] });
    let Kind::Smmuv3 { base, gsivs, proximity, id_mapping_index, .. } = &n[1].kind else {
        panic!()
    };
    assert_eq!(
        (*base, *gsivs, *proximity, *id_mapping_index),
        (0x4000_0000, [200, 201, 202, 203], None, Some(0))
    );
    let Kind::RootComplex { segment, ats, address_limit, .. } = n[2].kind else { panic!() };
    assert_eq!((segment, ats, address_limit), (0, 1, Some(48)));
    let Kind::NamedComponent { name, address_limit, .. } = &n[3].kind else { panic!() };
    assert_eq!((name.as_slice(), *address_limit), (&b"\\_SB.ETH0"[..], 44));
    assert_eq!(
        translate(&n, RC, 0x42),
        Some((ITS, 0x1_0142)),
        "requester ID to stream ID to device ID"
    );
    assert_eq!(translate(&n, RC, 0x100), None, "outside the root complex's range");
    assert_eq!(
        translate(&n, NAMED, 5),
        Some((ITS, 0x1_2000)),
        "a single mapping: the node's own ID"
    );
    assert_eq!(translate(&n, 1, 0), None, "no node there");
}
