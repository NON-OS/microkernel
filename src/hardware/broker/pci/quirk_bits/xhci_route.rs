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

//! Intel xHCI port routing (H-10). On the 7, 8 and 9 series PCHs firmware
//! leaves the shared USB 2.0 ports routed to EHCI and SuperSpeed off until
//! the OS writes XUSB2PR (0xD0) and USB3_PSSEN (0xD8); the ports the OS may
//! move are the ones firmware sets in USB2PRM (0xD4) and USB3PRM (0xDC). A
//! driver of the claimed xHCI may set or clear those bits, and no other,
//! while no EHCI on the same PCH is claimed. Pure: kernel_proofs.
//!
//! Sources, read for the facts alone (no code taken), Linux at
//! 21a7f44fd076: the device ids from include/linux/pci_ids.h:2786
//! (PANTHERPOINT_XHCI 0x1e31) and drivers/usb/host/xhci-pci.c:47-49
//! (LYNXPOINT_XHCI 0x8c31, LYNXPOINT_LP_XHCI 0x9c31, WILDCATPOINT_LP_XHCI
//! 0x9cb1); the offsets from drivers/usb/host/pci-quirks.c:74-77. An id not
//! found there is not listed. Unverified on hardware.

use super::Ident;

/// The Intel xHCI functions whose ports route between it and EHCI.
pub(crate) const SWITCHABLE: [u16; 4] = [0x1E31, 0x8C31, 0x9C31, 0x9CB1];
pub(crate) const XUSB2PR: u32 = 0xD0;
pub(crate) const USB3_PSSEN: u32 = 0xD8;
const INTEL: u16 = 0x8086;

/// The bits at `offset` a driver of xHCI `id` may change, or `None`.
pub(crate) fn xhci_route(id: &Ident, offset: u32) -> Option<u16> {
    let xhci = id.class == 0x0C && id.subclass == 0x03 && id.prog_if == 0x30;
    if id.vendor != INTEL || !xhci || !SWITCHABLE.contains(&id.device) {
        return None;
    }
    if id.route.ehci_claimed {
        return None; // another capsule's EHCI drives those ports
    }
    match offset {
        XUSB2PR => Some(id.route.usb2prm),
        USB3_PSSEN => Some(id.route.usb3prm),
        _ => None,
    }
}

/// Whether the function at `other`, of `class` (class, subclass and
/// interface, 24 bits), claimed or not, keeps the xHCI at `xhci` from
/// routing: an EHCI on the same segment and bus that someone holds.
pub(crate) fn blocks(xhci: (u16, u8), other: (u16, u8), class: u32, claimed: bool) -> bool {
    other == xhci && class == 0x0C_0320 && claimed
}
