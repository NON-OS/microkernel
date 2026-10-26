// NONOS Operating System (AGPL-3.0-or-later)
//! Which regions the loader cleans for the kernel, and when a boot's
//! entropy may stand.

use crate::handed::handed_over;
use crate::strong_rule::{strong_enough, Strong};

#[test]
fn only_the_loaders_own_memory_types_are_handed_over() {
    // UEFI 2.10, EFI_MEMORY_TYPE: 1 LoaderCode, 2 LoaderData.
    let handed: Vec<u32> = (0..16).filter(|&t| handed_over(t)).collect();
    assert_eq!(handed, [1, 2]);
}

#[test]
fn a_demanding_platform_needs_a_strong_source() {
    let s = |firmware, cpu_draws| Strong { firmware, cpu_draws };
    assert!(!strong_enough(true, s(false, 0)), "counter and clock alone");
    assert!(strong_enough(true, s(true, 0)), "EFI_RNG alone");
    assert!(strong_enough(true, s(false, 1)), "one RNDR word");
    assert!(strong_enough(false, s(false, 0)), "x86_64 keeps its own rule");
}
