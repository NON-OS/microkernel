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

//! Finding the Bluetooth interface in a configuration descriptor. Its class
//! is Wireless Controller (0xE0), subclass RF Controller (0x01), protocol
//! Bluetooth Programming (0x01) (Vol 4, Part B, 3). Alternate setting 0 of
//! that interface holds the event and ACL endpoints; the isochronous
//! interface for voice is not used by an LE-only host.

use super::endpoint::{classify, Endpoint};
use crate::error::HciError;
use nonos_bt_wire::WireError;

pub const CLASS: [u8; 3] = [0xe0, 0x01, 0x01];
const INTERFACE: u8 = 4;
const ENDPOINT: u8 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BtInterface {
    pub number: u8,
    pub events: Endpoint,
    pub acl_in: Endpoint,
    pub acl_out: Endpoint,
}

/// The first Bluetooth interface whose alternate setting 0 has all three
/// endpoints. Descriptors are walked by their own lengths (USB 2.0, 9.5);
/// one that is shorter than its header or runs past the end is refused.
pub fn find(config: &[u8]) -> Result<BtInterface, HciError> {
    let (mut at, mut current) = (0, None::<u8>);
    let mut found: [Option<Endpoint>; 3] = [None; 3];
    while at < config.len() {
        let len = usize::from(config[at]);
        let d = config.get(at..at + len).filter(|_| len >= 2);
        let d = d.ok_or(HciError::Wire(WireError::Length))?;
        match d[1] {
            INTERFACE if len >= 9 => {
                if let (Some(number), [Some(e), Some(i), Some(o)]) = (current, found) {
                    return Ok(BtInterface { number, events: e, acl_in: i, acl_out: o });
                }
                let ours = d[5..8] == CLASS && d[3] == 0;
                (current, found) = (ours.then_some(d[2]), [None; 3]);
            }
            ENDPOINT if current.is_some() => {
                if let Some((role, ep)) = classify(d) {
                    found[role as usize].get_or_insert(ep);
                }
            }
            _ => {}
        }
        at += len;
    }
    match (current, found) {
        (Some(number), [Some(e), Some(i), Some(o)]) => {
            Ok(BtInterface { number, events: e, acl_in: i, acl_out: o })
        }
        _ => Err(HciError::NotBluetooth),
    }
}
