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

//! HEALTH's and LINK's replies, which only report.

/// A place in BONDS's list that names no bond.
pub const NO_PLACE: u8 = 0xFF;

/// HEALTH: the version the capsule speaks, and whether a controller is up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health {
    pub version: u16,
    pub controller: bool,
}

impl Health {
    pub fn read(data: &[u8]) -> Option<Self> {
        let [v0, v1, up] = *data else { return None };
        (up <= 1).then_some(Self { version: u16::from_le_bytes([v0, v1]), controller: up == 1 })
    }

    pub fn write(&self) -> [u8; 3] {
        let v = self.version.to_le_bytes();
        [v[0], v[1], u8::from(self.controller)]
    }
}

/// LINK: the state (`ops::NO_CONTROLLER` to `ops::READY`), the flags, and
/// the link's bond as its place in BONDS's list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Link {
    pub state: u8,
    pub flags: u8,
    pub place: u8,
}

impl Link {
    pub const KEYBOARD: u8 = 1 << 0;
    pub const MOUSE: u8 = 1 << 1;
    /// HID posts its keys to the input ring.
    pub const KEYS_POST: u8 = 1 << 2;
    pub const SCANNING: u8 = 1 << 3;
    /// A pairing window is open (OP_PAIR).
    pub const WINDOW: u8 = 1 << 4;
    /// Pairing waits on the user: OP_PASSKEY says for what.
    pub const WAITING: u8 = 1 << 5;

    /// A state or a flag v1 does not define is not a LINK reply.
    pub fn read(data: &[u8]) -> Option<Self> {
        let [state, flags, place] = *data else { return None };
        let known = state <= super::ops::READY && flags < 1 << 6;
        known.then_some(Self { state, flags, place })
    }

    pub fn write(&self) -> [u8; 3] {
        [self.state, self.flags, self.place]
    }

    pub fn has(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}
