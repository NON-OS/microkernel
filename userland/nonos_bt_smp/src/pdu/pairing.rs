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

//! Pairing Request and Pairing Response (Vol 3, Part H, 3.5.1 and 3.5.2):
//! IO capability, OOB data flag, AuthReq, maximum encryption key size, and
//! the keys each side will distribute.

use crate::method::IoCap;
use nonos_bt_wire::WireError;

/// AuthReq bits (Vol 3, Part H, 3.5.1).
pub mod auth {
    pub const BONDING: u8 = 0x01;
    pub const MITM: u8 = 0x04;
    pub const SC: u8 = 0x08;
    pub const KEYPRESS: u8 = 0x10;
}

/// Key distribution bits (Vol 3, Part H, 3.6.1).
pub mod keys {
    pub const ENC: u8 = 0x01;
    pub const ID: u8 = 0x02;
    pub const SIGN: u8 = 0x04;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pairing {
    pub io_cap: IoCap,
    pub oob: bool,
    pub auth_req: u8,
    pub max_key_size: u8,
    pub init_keys: u8,
    pub resp_keys: u8,
}

impl Pairing {
    /// From the six parameter octets. The key size must be 7 to 16 and the
    /// OOB flag 0 or 1; reserved AuthReq and key bits are kept, not
    /// refused, as a later version may define them.
    pub fn parse(p: &[u8]) -> Result<Self, WireError> {
        let &[io, oob, auth_req, max_key_size, init_keys, resp_keys] = p else {
            return Err(WireError::Length);
        };
        if !(7..=16).contains(&max_key_size) || oob > 1 {
            return Err(WireError::Reserved);
        }
        let io_cap = IoCap::from_wire(io)?;
        Ok(Self { io_cap, oob: oob == 1, auth_req, max_key_size, init_keys, resp_keys })
    }

    pub fn params(&self) -> [u8; 6] {
        let oob = self.oob as u8;
        [self.io_cap.wire(), oob, self.auth_req, self.max_key_size, self.init_keys, self.resp_keys]
    }

    /// The IOcap octets f6 takes: AuthReq, OOB flag, IO capability, most
    /// significant first (Vol 3, Part H, 2.2.8).
    pub fn f6_io_cap(&self) -> [u8; 3] {
        [self.auth_req, self.oob as u8, self.io_cap.wire()]
    }
}
