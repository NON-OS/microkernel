// NONOS Operating System (AGPL-3.0-or-later)
//! The token PIDs a device sees (USB 2.0, as recalled): the device model's
//! own, so the OHCI proofs, whose controller names no PIDs, mount it too.

pub const PID_SETUP: u8 = 0x2D;
pub const PID_IN: u8 = 0x69;
pub const PID_OUT: u8 = 0xE1;
