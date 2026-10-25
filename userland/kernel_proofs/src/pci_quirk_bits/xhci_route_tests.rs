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

//! H-10: an Intel 7/8/9 series xHCI's driver routes its ports, set or clear,
//! within firmware's masks, on the cited ids, never while an EHCI on its bus
//! is held; through its own claim (pci/write.rs), so a clear is its own too.

use super::quirk_bits::{blocks, only, writable, Ident, Route};

const CITED: [u16; 4] = [0x1E31, 0x8C31, 0x9C31, 0x9CB1];
const XUSB2PR: u32 = 0xD0;
const USB3_PSSEN: u32 = 0xD8;

fn xhci(vendor: u16, device: u16, prog_if: u8, ehci_claimed: bool) -> Ident {
    let (route, before) = (Route { usb2prm: 0x3FFF, usb3prm: 0x003F, ehci_claimed }, [0xFF; 3]);
    Ident { vendor, device, class: 0x0C, subclass: 0x03, prog_if, pcie_cap: None, before, route }
}

#[test]
fn each_cited_id_may_move_exactly_the_bits_firmware_masks() {
    for id in CITED.map(|d| xhci(0x8086, d, 0x30, false)) {
        assert_eq!(writable(&id, XUSB2PR), Some(0x3FFF), "{:#x}", id.device);
        assert_eq!(writable(&id, USB3_PSSEN), Some(0x003F), "{:#x}", id.device);
        assert!(only(0x3FFF, 0x3FFF, 0), "every routable port to the xHCI");
        assert!(!only(0x3FFF, 0x7FFF, 0), "a port firmware does not let go");
        assert!(only(0x3FFF, 0, 0x3FFF), "handed back, a clear");
    }
}

#[test]
fn an_id_not_cited_another_vendors_and_every_other_function_stay_refused() {
    for d in [0x8CB1, 0xA12F, 0x1E26, 0x0000] {
        // 0x8CB1, recalled for the 9 series desktop, is in neither source.
        assert_eq!(writable(&xhci(0x8086, d, 0x30, false), XUSB2PR), None, "{d:#x}");
    }
    for id in [xhci(0x1B36, 0x8C31, 0x30, false), xhci(0x8086, 0x8C31, 0x20, false)] {
        for at in [XUSB2PR, USB3_PSSEN] {
            assert_eq!(writable(&id, at), None, "{:#x} at {at:#x}", id.vendor);
        }
    }
    let listed = xhci(0x8086, 0x8C31, 0x30, false);
    for at in [0xD4, 0xDC, 0xD2, 0xDA, 0x04] {
        assert_eq!(writable(&listed, at), None, "the masks and their neighbours: {at:#x}");
    }
}

#[test]
fn while_an_ehci_on_its_bus_is_held_nothing_routes() {
    let held = xhci(0x8086, 0x9C31, 0x30, true);
    assert_eq!((writable(&held, XUSB2PR), writable(&held, USB3_PSSEN)), (None, None));
}

#[test]
fn only_a_claimed_ehci_on_the_same_segment_and_bus_blocks() {
    let at = (0, 0);
    assert!(blocks(at, (0, 0), 0x0C_0320, true));
    assert!(!blocks(at, (0, 0), 0x0C_0320, false), "nobody holds it");
    assert!(!blocks(at, (0, 1), 0x0C_0320, true), "another bus");
    assert!(!blocks(at, (1, 0), 0x0C_0320, true), "another segment");
    assert!(!blocks(at, (0, 0), 0x0C_0330, true), "the xHCI itself, or another xHCI");
    assert!(!blocks(at, (0, 0), 0x0C_0310, true), "OHCI is no companion here");
}
