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

//! Discovery finds an Intel Wi-Fi adapter whatever firmware left in its INTx
//! registers, and finds nothing else; a refused interrupt bind ends the
//! attempt only when the claim is gone. Laptops whose firmware leaves the
//! legacy line at 0xFF (the OS is expected to use MSI/MSI-X) used to have
//! their adapter skipped at discovery.

use crate::irq_plan::{after_intx, after_msix, Refusal};
use crate::pci_match::{intx_usable, is_supported_adapter, Candidate};

// An AX210 (0x2725): class network/other, BAR0 a memory BAR.
fn ax210() -> Candidate {
    Candidate {
        is_pci: true,
        vendor: 0x8086,
        device: 0x2725,
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
fn every_supported_family_is_found() {
    for device in [0x095Au16, 0x24FD, 0x2526, 0x2723, 0x2725, 0x51F0] {
        assert!(is_supported_adapter(&Candidate { device, ..ax210() }), "{device:#06x}");
    }
}

#[test]
fn every_platform_the_gen3_path_has_firmware_for_is_found() {
    // A device the gen3 selection would boot must not be skipped at
    // discovery: every id with SO transport values is a supported adapter.
    for device in 0..=u16::MAX {
        if crate::gen3::select::transport(device).is_some() {
            assert!(is_supported_adapter(&Candidate { device, ..ax210() }), "{device:#06x}");
        }
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

#[test]
fn only_a_lost_claim_ends_the_interrupt_ladder() {
    // ESTALE: the claim epoch is gone, nothing more can be bound.
    assert_eq!(after_intx(-116), Refusal::ClaimLost);
    assert_eq!(after_msix(-116), Refusal::ClaimLost);
    // EPERM on INTx can be a kernel-reserved line: MSI-X is still worth trying.
    assert_eq!(after_intx(-1), Refusal::TryNext);
    // EPERM on MSI-X means the device is not ours.
    assert_eq!(after_msix(-1), Refusal::ClaimLost);
    // No INTx, no MSI-X capability, a busy line: move on, finally run polled.
    for rc in [-22i64, -16, -12, -19, -95] {
        assert_eq!(after_intx(rc), Refusal::TryNext, "intx {rc}");
        assert_eq!(after_msix(rc), Refusal::TryNext, "msix {rc}");
    }
}
