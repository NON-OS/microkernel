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

//! The lies the air's controller can tell once a test turns them on: a
//! link it forgets without Disconnection Complete (as one that reset itself
//! would), ACL buffers it never counts back, and commands it never answers.

use super::state::{Air, HANDLE};
use crate::hci_sim::events::complete;
use nonos_bt_hci::host::READ_RSSI;

#[derive(Default)]
pub struct Lies {
    pub forgot: bool,
    pub withhold: bool,
    pub mute: bool,
}

/// Unknown Connection Identifier (Vol 1, Part F, 2).
const NO_SUCH_LINK: u8 = 0x02;

impl Air {
    /// Read RSSI (Vol 4, Part E, 7.5.4) on the one link, -60 dBm.
    pub(super) fn rssi(&self) -> Vec<Vec<u8>> {
        if self.lies.forgot {
            return vec![complete(1, READ_RSSI.0, NO_SUCH_LINK, &[])];
        }
        let [h0, h1] = HANDLE.to_le_bytes();
        vec![complete(1, READ_RSSI.0, 0, &[h0, h1, -60i8 as u8])]
    }
}
