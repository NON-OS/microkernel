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

//! H-10: where the routing rule's inputs are read, in the files the host
//! cannot run: at the time of the write, and a claim at any stage counting.

const IDENT: &str = include_str!("../../../../src/hardware/broker/pci/ident_read.rs");
const COMPANION: &str = include_str!("../../../../src/hardware/broker/pci/companion.rs");
const CLAIMED: &str = include_str!("../../../../src/hardware/broker/claim/claimed.rs");
const WRITE: &str = include_str!("../../../../src/hardware/broker/pci/write.rs");

#[test]
fn the_masks_and_the_companion_are_read_at_the_write_for_an_intel_xhci_only() {
    let intel = IDENT.find("if vendor == 0x8086 && class >> 8 == 0x0C_0330 {").expect("unguarded");
    let body = &IDENT[intel..];
    assert!(body.contains("Route { usb2prm: mask(0xD4), usb3prm: mask(0xDC), ehci_claimed }"));
    assert!(body.contains("let ehci_claimed = super::companion::ehci_claimed(at);"));
    assert!(body.contains("} else {\n        Route::NONE\n    };"));
    // ident is read after the writer's own claim is resolved, under it.
    assert!(
        WRITE.find("resolve(pid, &req)?").unwrap()
            < WRITE.find("ident(&cfg, register as u32)?").unwrap()
    );
}

#[test]
fn any_claim_on_an_ehci_on_the_bus_counts_whatever_its_stage() {
    assert!(COMPANION.contains("let claimed = claim::claimed(h.device_id);"));
    assert!(COMPANION.contains("blocks(at, (h.address.segment, h.address.bus), class, claimed)"));
    assert!(CLAIMED.contains("claims().iter().any(|c| c.device_id == device_id)"));
    assert!(
        !CLAIMED.contains(".stage"),
        "a stage filter would let a claim on its way in go unseen"
    );
}
