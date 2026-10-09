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

//! Hybrid core decoding, against the CPUID.1AH values the Intel SDM lists
//! and what Alder Lake and Raptor Lake parts answer.

use super::core_kind::{CoreKind, LEAF7_EDX_HYBRID, LEAF_CORE_TYPE};

#[test]
fn leaf_1a_names_core_and_atom_by_the_top_byte_only() {
    // EAX[23:0] is the native model id and must not change the answer.
    assert_eq!(CoreKind::from_leaf_1a(0x4000_0001), CoreKind::Performance);
    assert_eq!(CoreKind::from_leaf_1a(0x2000_0001), CoreKind::Efficiency);
    assert_eq!(CoreKind::from_leaf_1a(0x40FF_FFFF), CoreKind::Performance);
    assert_eq!(CoreKind::from_leaf_1a(0), CoreKind::Unknown);
    assert_eq!(CoreKind::from_leaf_1a(0x1000_0000), CoreKind::Unknown);
}

#[test]
fn the_hybrid_bit_and_the_core_type_leaf_are_the_sdms() {
    assert_eq!(LEAF7_EDX_HYBRID, 0x0000_8000);
    assert_eq!(LEAF_CORE_TYPE, 0x1A);
}

#[test]
fn a_kind_survives_its_stored_byte() {
    for kind in [CoreKind::Unknown, CoreKind::Performance, CoreKind::Efficiency] {
        assert_eq!(CoreKind::from_u8(kind as u8), kind);
    }
    assert_eq!(CoreKind::from_u8(0xFF), CoreKind::Unknown);
}

#[test]
fn an_idle_p_core_is_offered_a_wake_before_an_e_core() {
    assert_eq!(CoreKind::Performance.wake_pass(), 0);
    assert_eq!(CoreKind::Unknown.wake_pass(), 0);
    assert_eq!(CoreKind::Efficiency.wake_pass(), 1);
    assert_eq!(CoreKind::Efficiency.letter(), b"E");
}
