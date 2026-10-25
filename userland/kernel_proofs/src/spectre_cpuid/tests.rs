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

use super::cpuid_spec::{decode, SpecControls};

const NONE: SpecControls = SpecControls { ibrs: false, ibpb: false, stibp: false, ssbd: false };
const ALL: SpecControls = SpecControls { ibrs: true, ibpb: true, stibp: true, ssbd: true };

#[test]
fn an_intel_part_reports_its_controls_in_leaf_7() {
    let edx = (1 << 26) | (1 << 27) | (1u32 << 31);
    assert_eq!(decode(edx, 0), ALL);
}

#[test]
fn an_amd_part_reports_its_controls_in_leaf_8000_0008() {
    let ebx = (1 << 12) | (1 << 14) | (1 << 15) | (1 << 24);
    assert_eq!(decode(0, ebx), ALL);
}

#[test]
fn amd_ibpb_without_ibrs_still_gives_the_switch_its_barrier() {
    assert_eq!(decode(0, 1 << 12), SpecControls { ibpb: true, ..NONE });
}

#[test]
fn each_amd_bit_names_one_control() {
    assert_eq!(decode(0, 1 << 14), SpecControls { ibrs: true, ..NONE });
    assert_eq!(decode(0, 1 << 15), SpecControls { stibp: true, ..NONE });
    assert_eq!(decode(0, 1 << 24), SpecControls { ssbd: true, ..NONE });
}

#[test]
fn bits_that_are_not_these_controls_report_nothing() {
    assert_eq!(decode(!((1 << 26) | (1 << 27) | (1u32 << 31)), 0), NONE);
    assert_eq!(decode(0, !((1 << 12) | (1 << 14) | (1 << 15) | (1 << 24))), NONE);
}

#[test]
fn this_host_reads_the_same_controls_its_raw_leaves_decode_to() {
    use core::arch::x86_64::{__cpuid, __cpuid_count};
    let leaf7 = if __cpuid(0).eax >= 7 { __cpuid_count(7, 0).edx } else { 0 };
    let ext8 = if __cpuid(0x8000_0000).eax >= 0x8000_0008 { __cpuid(0x8000_0008).ebx } else { 0 };
    assert_eq!(super::cpuid_spec::read(), decode(leaf7, ext8));
}
