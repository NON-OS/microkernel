// NONOS Operating System (AGPL-3.0-or-later)

//! Which PCI functions discovery finds, whatever their INTx registers say.

use crate::pci_match::{intx_usable, is_supported_adapter, Candidate};

// An AX211 on an SO platform (0x51F0): class network/other, BAR0 a memory BAR.
fn ax210() -> Candidate {
    Candidate {
        is_pci: true,
        vendor: 0x8086,
        device: 0x51F0,
        class: 0x02,
        subclass: 0x80,
        bar0_mmio: true,
    }
}

#[test]
fn an_adapter_with_no_routed_intx_line_is_still_found() {
    // The record carries irq_pin 0 and irq_line 0xFF; neither is an input to
    // the decision, and the bind reads them as "no INTx" rather than skipping.
    assert!(is_supported_adapter(&ax210()), "the adapter is found");
    assert!(!intx_usable(0, 0xFF), "pin 0 / line 0xFF is no usable INTx line");
    assert!(!intx_usable(1, 0xFF), "an unrouted line is not usable");
    assert!(!intx_usable(0, 11), "no pin means no INTx");
    assert!(intx_usable(1, 11), "a routed line is tried first");
}

#[test]
fn a_part_this_driver_cannot_run_is_not_claimed() {
    // W4: 7265, 8265, 9260, 9560, AX200, AX201, the discrete AX210 and
    // Meteor Lake. Each is still named at discovery (`announce`).
    for device in [0x095Au16, 0x24FD, 0x2526, 0x9DF0, 0x2723, 0xA0F0, 0x2725, 0x7E40, 0x2729] {
        assert!(!is_supported_adapter(&Candidate { device, ..ax210() }), "{device:#06x}");
    }
}

#[test]
fn every_platform_the_gen3_path_has_firmware_for_is_found() {
    // A device the gen3 path boots must not be skipped at discovery, and no
    // other is claimed: exactly the ids whose firmware is carried are found.
    for device in 0..=u16::MAX {
        let found = is_supported_adapter(&Candidate { device, ..ax210() });
        assert_eq!(found, crate::firmware::carried::runs(device), "{device:#06x}");
    }
    for device in [0x7A70u16, 0x7AF0, 0x7F70] {
        assert!(is_supported_adapter(&Candidate { device, ..ax210() }), "{device:#06x}");
    }
}

#[test]
fn a_non_intel_or_non_network_function_is_not_found() {
    assert!(!is_supported_adapter(&Candidate { vendor: 0x10EC, ..ax210() }), "Realtek");
    assert!(!is_supported_adapter(&Candidate { class: 0x04, subclass: 0x03, ..ax210() }), "audio");
    assert!(!is_supported_adapter(&Candidate { subclass: 0x00, ..ax210() }), "ethernet");
    assert!(!is_supported_adapter(&Candidate { device: 0x15F3, ..ax210() }), "an Intel NIC id");
    assert!(!is_supported_adapter(&Candidate { device: 0x272B, ..ax210() }), "BE200, not run");
    assert!(!is_supported_adapter(&Candidate { is_pci: false, ..ax210() }), "not on PCI");
    assert!(!is_supported_adapter(&Candidate { bar0_mmio: false, ..ax210() }), "no register BAR");
}
