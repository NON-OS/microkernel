// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Which USB Bluetooth controllers may be Realtek's. Linux v6.12's btusb.c
//! marks these BTUSB_REALTEK for the RTL8821C: 0x13d3:0x3529 by its own id
//! ("Realtek 8821CE Bluetooth devices"), and every device of vendor 0x0bda
//! with the Bluetooth interface class E0/01/01
//! (USB_VENDOR_AND_INTERFACE_INFO), which takes the ids RTL8821CE combo
//! cards report, 0x0bda:0xb00a and 0x0bda:0xc821 among them. The capsule
//! only reaches this with a device whose Bluetooth interface it found, so
//! the class is already E0/01/01. Which chip it is, Read Local Version
//! says next (`chip`): an id alone sends no vendor command.

/// Realtek Semiconductor's USB vendor id.
pub const REALTEK: u16 = 0x0bda;

/// RTL8821C controllers btusb.c lists under another vendor's id.
pub const RTL8821C_OTHER_VENDORS: [(u16, u16); 1] = [(0x13d3, 0x3529)];

/// Ids RTL8821CE cards report under Realtek's own vendor id; taken by the
/// vendor match, listed for the record and the proofs.
pub const RTL8821C_REALTEK_PRODUCTS: [u16; 2] = [0xb00a, 0xc821];

/// Whether this Bluetooth controller may be a Realtek one btusb.c patches.
pub fn realtek(vendor: u16, product: u16) -> bool {
    vendor == REALTEK || RTL8821C_OTHER_VENDORS.contains(&(vendor, product))
}
