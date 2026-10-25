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

use super::apic_constants as k;
use super::apic_plan::*;
use super::idt_vectors as v;

/* ---- APIC id from CPUID ---- */

#[test]
fn apic_id_uses_the_32_bit_x2apic_id_when_leaf_0b_exists() {
    // x2APIC id 0x104 on a many-core part: leaf 1 only knows 0x04.
    assert_eq!(apic_id_from_cpuid(0x1F, 0x0002, 0x104, 0x0410_0800), 0x104);
    // Hybrid E-core id 0x4F, same either way.
    assert_eq!(apic_id_from_cpuid(0x20, 0x0001, 0x4F, 0x4F10_0800), 0x4F);
    // No leaf 0x0B (or an empty one): fall back to leaf 1.
    assert_eq!(apic_id_from_cpuid(0x0A, 0, 0, 0x0310_0800), 3);
    assert_eq!(apic_id_from_cpuid(0x0D, 0, 0xDEAD, 0x0210_0800), 2);
}

#[test]
fn x2apic_is_required_only_past_0xfe() {
    assert!(!requires_x2apic(&[0, 2, 4, 6, 0x4F, 0xFE]));
    assert!(requires_x2apic(&[0, 0xFF]));
    assert!(requires_x2apic(&[0, 0x100]));
}

/* ---- device interrupt destinations without remapping ---- */

#[test]
fn device_irqs_go_to_a_cpu_an_8_bit_field_can_name() {
    assert_eq!(pick_irq_dest(0x10, [0]), Some(0x10));
    assert_eq!(pick_irq_dest(0x104, [0x100, 0x20, 0]), Some(0x20));
    assert_eq!(pick_irq_dest(0x104, [0x100, 0x101]), None);
    assert_eq!(pick_irq_dest(0xFF, [0]), Some(0), "0xFF is broadcast, not a CPU");
}

/* ---- vectors ---- */

#[test]
fn lapic_vectors_have_gates_and_do_not_alias_device_lines() {
    assert_eq!(k::VEC_SPURIOUS, v::VECTOR_APIC_SPURIOUS);
    assert_eq!(k::VEC_ERROR, v::VECTOR_APIC_ERROR);
    assert_eq!(k::VEC_THERMAL, v::VECTOR_APIC_THERMAL);
    for vec in [k::VEC_ERROR, k::VEC_THERMAL, k::VEC_SPURIOUS] {
        assert!(!(v::IRQ_VECTOR_START..=v::IRQ_VECTOR_END).contains(&vec));
    }
    assert_eq!(k::VEC_SPURIOUS & 0x0F, 0x0F, "SVR vector low nibble set on P6");
}
