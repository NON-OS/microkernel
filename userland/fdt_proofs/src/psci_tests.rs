// NONOS Operating System (AGPL-3.0-or-later)
//! The PSCI conduit: QEMU virt's "hvc", a Pi 4's absence, and every node the
//! kernel must not call through.

use crate::arch::fdt::find::psci::{find, Conduit};
use crate::arch::fdt::Fdt;
use crate::builder::{text, Builder, END};
use crate::fixtures::{RPI4B, VIRT_GICV2, VIRT_GICV3};

fn tree(compatible: &[&str], method: Option<&[u8]>, status: &str) -> Vec<u8> {
    let list: Vec<u8> = compatible.iter().flat_map(|c| text(c)).collect();
    let mut b = Builder::default().begin("").begin("psci").prop("compatible", &list);
    if let Some(method) = method {
        b = b.prop("method", method);
    }
    b = b.prop("status", &text(status));
    b.end().end().token(END).finish()
}

fn conduit(blob: &[u8]) -> Option<Conduit> {
    find(&Fdt::from_bytes(blob).unwrap()).unwrap()
}

#[test]
fn qemu_virt_at_el1_is_reached_by_hvc_and_a_pi_has_no_psci() {
    assert_eq!(conduit(VIRT_GICV3), Some(Conduit::Hvc));
    assert_eq!(conduit(VIRT_GICV2), Some(Conduit::Hvc));
    assert_eq!(conduit(RPI4B), None);
}

#[test]
fn smc_firmware_is_reached_by_smc() {
    let blob = tree(&["arm,psci-1.0", "arm,psci-0.2"], Some(&text("smc")), "okay");
    assert_eq!(conduit(&blob), Some(Conduit::Smc));
    let blob = tree(&["arm,psci-0.2"], Some(&text("hvc")), "okay");
    assert_eq!(conduit(&blob), Some(Conduit::Hvc));
}

#[test]
fn a_node_the_standard_calls_cannot_trust_is_none() {
    // PSCI 0.1 names its own function ids, which the kernel does not read.
    assert_eq!(conduit(&tree(&["arm,psci"], Some(&text("smc")), "okay")), None);
    assert_eq!(conduit(&tree(&["arm,psci-1.0"], None, "okay")), None);
    assert_eq!(conduit(&tree(&["arm,psci-1.0"], Some(&text("svc")), "okay")), None);
    // Not a string: no terminator.
    assert_eq!(conduit(&tree(&["arm,psci-1.0"], Some(b"smc"), "okay")), None);
    assert_eq!(conduit(&tree(&["arm,psci-1.0"], Some(&text("smc")), "disabled")), None);
}
