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
//! The events that open, secure and end a link, laid out by hand, and
//! the connection the proofs ask for.

use super::config::Config;
use super::events::status;
use nonos_bt_hci::host::{Connect, DEFAULT_LINK, DEFAULT_SCAN};
use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::hci::opcode::LE_ENABLE_ENCRYPTION;

/// The peer the link proofs connect to: a static random address, its two
/// top bits set (Vol 6, Part B, 1.3.2.1), least significant octet first.
pub const PEER: [u8; 6] = [0x01, 0x02, 0x03, 0x04, 0x05, 0xc6];

pub const CONNECT: Connect = Connect {
    scan: DEFAULT_SCAN,
    peer_type: AddrType::Random,
    peer: BdAddr(PEER),
    link: DEFAULT_LINK,
};

/// LE Connection Complete (Vol 4, Part E, 7.7.65.1) from `PEER`, as
/// central (role 0), interval 0x0020, latency 0, timeout 0x0190, clock
/// accuracy 0.
pub fn connected(status: u8, handle: u16) -> Vec<u8> {
    let [h0, h1] = handle.to_le_bytes();
    let mut e = vec![0x3e, 19, 0x01, status, h0, h1, 0x00, 0x01];
    e.extend_from_slice(&PEER);
    e.extend_from_slice(&[0x20, 0x00, 0x00, 0x00, 0x90, 0x01, 0x00]);
    e
}

/// Encryption Change (Vol 4, Part E, 7.7.8).
pub fn encryption_change(status: u8, handle: u16, enabled: u8) -> Vec<u8> {
    let [h0, h1] = handle.to_le_bytes();
    vec![0x08, 4, status, h0, h1, enabled]
}

/// Disconnection Complete (Vol 4, Part E, 7.7.5).
pub fn disconnected(handle: u16, reason: u8) -> Vec<u8> {
    let [h0, h1] = handle.to_le_bytes();
    vec![0x05, 4, 0x00, h0, h1, reason]
}

/// A controller that takes LE Enable Encryption and then sends `after`.
pub fn encrypt_answered(after: Vec<Vec<u8>>) -> Config {
    let start = LE_ENABLE_ENCRYPTION.0;
    let mut events = vec![status(0, 1, start)];
    events.extend(after);
    Config::scripted(vec![(start, events)])
}
