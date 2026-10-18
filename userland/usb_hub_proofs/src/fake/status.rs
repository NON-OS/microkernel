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

//! The four GET_STATUS bytes a modelled hub port answers (USB 2.0 table
//! 11-21, USB 3.2 table 10-10, as the decode under test reads them).

use super::model::HubModel;

impl HubModel {
    /// wPortStatus and wPortChange of `port` (from 1).
    pub fn status(&self, port: usize) -> [u8; 4] {
        let p = &self.ports[port - 1];
        let speed = p.device.as_ref().map_or(0, |d| d.speed);
        let mut st = p.device.is_some() as u16 | (p.enabled as u16) << 1;
        st |= ((p.resetting != 0) as u16) << 4;
        st |= (p.powered as u16) << if self.superspeed { 9 } else { 8 };
        if !self.superspeed && p.enabled {
            st |= ((speed == 2) as u16) << 9 | ((speed == 3) as u16) << 10;
        }
        let ch = p.c_connection as u16 | (p.c_reset as u16) << 4;
        let [a, b] = st.to_le_bytes();
        let [c, d] = ch.to_le_bytes();
        [a, b, c, d]
    }
}
