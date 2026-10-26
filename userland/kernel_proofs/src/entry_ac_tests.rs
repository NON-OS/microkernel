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

//! Every kernel entry the IDT installs clears RFLAGS.AC before its handler
//! runs, with an instruction every x86_64 part takes, and the kernel's own
//! STAC/CLAC window never executes on a CPU without SMAP. A gate left AC as
//! ring 3 set it, so a capsule could run every handler with SMAP off; and
//! STAC on a CPU without SMAP is #UD, which the old helper called a NOP.

const MACRO: &str = include_str!("../../../src/arch/x86_64/asm/clear_ac.inc");
const BODIES: [(&str, &str); 4] = [
    ("noerr", include_str!("../../../src/arch/x86_64/asm/tramp_noerr.inc")),
    ("err", include_str!("../../../src/arch/x86_64/asm/tramp_err.inc")),
    ("ctx", include_str!("../../../src/arch/x86_64/asm/tramp_ctx.inc")),
    ("common", include_str!("../../../src/arch/x86_64/asm/interrupt_common.S")),
];
const GATES: [(&str, &str); 4] = [
    ("broker", include_str!("../../../src/arch/x86_64/interrupt/broker/isr.rs")),
    ("lvt", include_str!("../../../src/arch/x86_64/interrupt/apic/lvt_gates.rs")),
    ("ipi", include_str!("../../../src/arch/interrupt_controller/ipi_dispatch/x86_64/gates.rs")),
    ("fatal", include_str!("../../../src/interrupts/isr/wrappers_fatal.rs")),
];
const NMI: &str = include_str!("../../../src/interrupts/isr/wrappers.rs");
const SMAP: &str = include_str!("../../../src/arch/user_access/smap.rs");

#[test]
fn the_macro_clears_ac_without_clac() {
    let body = &MACRO[MACRO.find(".macro CLEAR_AC").unwrap()..];
    assert!(body.contains("pushfq") && body.contains("popfq"));
    assert!(body.contains("and     qword ptr [rsp], ~(1 << 18)"));
    assert!(!body.contains("clac"));
}

#[test]
fn every_assembly_entry_clears_ac_after_cld() {
    for (name, src) in BODIES {
        assert!(src.contains("    cld\n    CLEAR_AC\n"), "{name}");
    }
}

#[test]
fn every_rust_gate_clears_ac_first() {
    for (name, src) in GATES {
        let gates = src.matches("extern \"x86-interrupt\" fn ").count();
        let clears = src.matches("clear_ac();").count();
        assert!(gates > 0 && gates == clears, "{name}: {gates} gates, {clears} clears");
    }
    let nmi = &NMI[NMI.find("fn isr_nmi(").unwrap()..];
    let body = &nmi[..nmi.find("handlers::nmi").unwrap()];
    assert!(body.contains("clear_ac();"));
}

#[test]
fn stac_and_clac_run_only_where_cpuid_lists_smap() {
    assert_eq!(SMAP.matches("asm!(\"stac\"").count(), 1);
    assert_eq!(SMAP.matches("asm!(\"clac\"").count(), 1);
    assert_eq!(SMAP.matches("if present() {").count(), 2);
    assert!(SMAP.contains("cpuid(7, 0).1 & (1 << 20)"));
}
