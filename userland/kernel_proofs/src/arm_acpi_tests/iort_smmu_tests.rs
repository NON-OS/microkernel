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

//! IORT's SMMUv1/v2 and reserved memory range nodes: every field and
//! interrupt read, IDs followed from the root complex through the SMMUv2
//! to the ITS, each reserved range tied to the stream IDs it serves on
//! the SMMU (not the ITS), and arrays or ranges that do not hold refused.

use super::hostile::every_cut_and_byte;
use super::iort_build::iort_of;
use super::iort_smmu_build::*;
use crate::arm_acpi::iort::{iort, reserved, translate, Irq, Kind, Reserved};

#[test]
fn an_smmu_v2_and_its_interrupts() {
    let n = iort(&iort_of(&nodes_v2(76, &RANGES))).unwrap();
    assert_eq!(n.iter().map(|x| x.offset).collect::<Vec<_>>(), [ITS, SMMU, RC, RMR]);
    let Kind::Smmuv2 { base, span, model, flags, global, context, pmu } = &n[1].kind else {
        panic!("{:?}", n[1].kind)
    };
    assert_eq!((*base, *span, *model, *flags), (0x1500_0000, 0x8_0000, 1, 1));
    let irq = |gsiv, edge| Irq { gsiv, edge };
    assert_eq!(global, &[irq(100, false), irq(101, true)]);
    assert_eq!(context, &[irq(110, false), irq(111, false), irq(112, true)]);
    assert_eq!(pmu, &[irq(120, true)]);
    assert!(translate(&n, RC, 0x42).is_some(), "through the SMMUv2 to the ITS");
    assert_eq!(n[3].kind, Kind::Rmr { flags: 0, ranges: RANGES.to_vec() });
}

#[test]
fn each_reserved_range_is_tied_to_its_streams_on_the_smmu() {
    let n = iort(&iort_of(&nodes_v2(76, &RANGES))).unwrap();
    let r = |first_stream, streams_less_one, (base, length): (u64, u64)| Reserved {
        smmu: SMMU,
        first_stream,
        streams_less_one,
        base,
        length,
    };
    let (a, b) = (RANGES[0], RANGES[1]);
    let want = [r(0x80, 1, a), r(0x80, 1, b), r(0x90, 0, a), r(0x90, 0, b)];
    assert_eq!(reserved(&n), want, "the mapping into the ITS names no stream");
}

#[test]
fn arrays_and_ranges_that_do_not_hold_are_refused() {
    assert!(iort(&iort_of(&nodes_v2(8, &RANGES))).is_err(), "context array inside the header");
    assert!(iort(&iort_of(&nodes_v2(120, &RANGES))).is_err(), "context array past the node");
    assert!(iort(&iort_of(&nodes_v2(76, &[(0x1000, 0)]))).is_err(), "an empty range");
    assert!(iort(&iort_of(&nodes_v2(76, &[(u64::MAX, 2)]))).is_err(), "a range that wraps");
    assert!(iort(&iort_of(&nodes_v2(76, &[]))).is_ok(), "no ranges at all is an empty list");
}

#[test]
fn hostile_bytes_never_panic() {
    every_cut_and_byte(&iort_of(&nodes_v2(76, &RANGES)), |b| iort(b).map(|n| reserved(&n).len()));
}
