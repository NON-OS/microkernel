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

//! What GET_CONNECTOR_STATUS answers (UCSI 1.2; bit offsets recalled,
//! unverified): the change bits (15:0), power operation mode (18:16),
//! connect status (19), power direction (20, set when the connector
//! sources power), partner flags (28:21), partner type (31:29), the
//! request data object (63:32), battery charging status (65:64).

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Status {
    pub change: u16,
    /// 1 USB default, 2 BC, 3 PD, 4 Type-C 1.5 A, 5 Type-C 3 A.
    pub power_mode: u8,
    pub connected: bool,
    /// Sourcing power to the partner; clear when sinking from it.
    pub provider: bool,
    pub partner_flags: u8,
    pub partner_type: u8,
    pub rdo: u32,
    /// 0 not charging, 1 nominal, 2 slow, 3 very slow.
    pub charging: u8,
}

impl Status {
    /// The connector's state, without which bits changed to reach it.
    pub fn state(&self) -> Status {
        Status { change: 0, ..*self }
    }

    /// A charger: a partner this connector sinks power from.
    pub fn charger(&self) -> bool {
        self.connected && !self.provider
    }
}

pub fn status(b: &[u8]) -> Option<Status> {
    let b = b.get(..9)?;
    let w = u16::from_le_bytes([b[2], b[3]]);
    Some(Status {
        change: u16::from_le_bytes([b[0], b[1]]),
        power_mode: (w & 7) as u8,
        connected: w & (1 << 3) != 0,
        provider: w & (1 << 4) != 0,
        partner_flags: (w >> 5) as u8,
        partner_type: (w >> 13) as u8,
        rdo: u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
        charging: b[8] & 3,
    })
}
