// NONOS Operating System (AGPL-3.0-or-later)
//! The firmware commands the join proofs name, and the orders they run in.

pub(super) const PHY: (u8, u8) = (0x1, 0x08);
pub(super) const RLC: (u8, u8) = (0x5, 0x08);
pub(super) const LINK: (u8, u8) = (0x3, 0x09);
pub(super) const STA_CFG: (u8, u8) = (0x3, 0x0A);
pub(super) const STA_RM: (u8, u8) = (0x3, 0x0C);
pub(super) const QUEUE: (u8, u8) = (0x5, 0x17);
pub(super) const SESSION: (u8, u8) = (0x3, 0x05);
pub(super) const MAC: (u8, u8) = (0x3, 0x08);
pub(super) const KEY: (u8, u8) = (0x5, 0x18);
pub(super) const FLUSH: (u8, u8) = (0x1, 0x1E);

/// Up to the open port, in Linux's order.
pub(super) const JOIN_COMMANDS: [(u8, u8); 15] = [
    PHY, RLC, LINK, LINK, STA_CFG, QUEUE, QUEUE, SESSION, MAC, STA_CFG, LINK, KEY, KEY, STA_CFG,
    SESSION,
];

/// Taking down contexts that reached the queues and the session.
pub(super) const TEARDOWN_FROM_SESSION: [(u8, u8); 7] =
    [SESSION, FLUSH, QUEUE, QUEUE, STA_RM, LINK, PHY];
