// NONOS Operating System (AGPL-3.0-or-later)
//! The tree decides when it names a GIC; the CPU decides when it does not;
//! a GICv3 tree on a CPU without the registers is refused.

use crate::gic::select::select;
use crate::gic::version::GicVersion::{V2, V3};

#[test]
fn the_device_tree_decides_when_it_names_one() {
    assert_eq!(select(Some(V2), true), Ok(V2));
    assert_eq!(select(Some(V2), false), Ok(V2));
    assert_eq!(select(Some(V3), true), Ok(V3));
}

#[test]
fn a_gicv3_tree_needs_the_system_registers() {
    assert!(select(Some(V3), false).is_err());
}

#[test]
fn without_a_tree_the_cpu_decides() {
    // QEMU `virt,gic-version=2` hands the CPU no GICv3 interface, so its
    // ID_AA64PFR0_EL1.GIC reads zero and the GICv2 backend is the one taken.
    assert_eq!(select(None, false), Ok(V2));
    assert_eq!(select(None, true), Ok(V3));
}
