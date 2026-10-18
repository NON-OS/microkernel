// NONOS Operating System (AGPL-3.0-or-later)
//! The hardware id, what the GIC compares and the descriptors are filled
//! from, is the whole packed affinity: Aff3 included, so a part with Aff3
//! set still matches its own descriptor.

use crate::users_tests::read;

#[test]
fn the_hardware_id_keeps_aff3() {
    let hw = read("arch/aarch64/abi/cpu_id.rs");
    assert!(hw.contains(
        "pub(super) fn current() -> u32 {\ncrate::arch::aarch64::cpu::cpu_affinity()\n}"
    ));
    let affinity = read("arch/aarch64/cpu/id/affinity.rs");
    assert!(affinity.contains("pub fn cpu_affinity() -> u32 {\npack_affinity(mpidr())\n}"));
    assert!(affinity.contains("let aff3 = ((mpidr_affinity >> 32) & 0xFF) as u32;"));
    assert!(affinity.contains("(aff3 << 24) | (aff2 << 16) | (aff1 << 8) | aff0"));
}
