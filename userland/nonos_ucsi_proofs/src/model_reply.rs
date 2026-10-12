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

//! What the model PPM answers once a command's delay has passed.

use crate::command::*;
use crate::model::Model;

const CC: u32 = 1 << 31;
const ERROR: u32 = 1 << 30;
const ACK: u32 = 1 << 29;
const RESET: u32 = 1 << 27;
const NOT_SUPPORTED: u32 = 1 << 25;

impl Model {
    pub(crate) fn advance(&mut self) {
        let Some((control, polls)) = self.control else { return };
        if self.stuck || polls < self.delay {
            self.control = Some((control, polls + 1));
            return;
        }
        self.control = None;
        let (code, n) = (control as u8, ((control >> 16) & 0x7F) as usize);
        self.message = [0; 16];
        self.cci = match code {
            PPM_RESET => {
                (self.notify_mask, self.cci) = (0, 0);
                RESET
            }
            ACK_CC_CI => {
                if control >> 16 & 1 != 0 {
                    self.changes.pop_front();
                }
                ACK
            }
            SET_NOTIFICATION_ENABLE => {
                self.notify_mask = (control >> 16) as u16;
                CC
            }
            GET_CAPABILITY => {
                self.message[4] = self.ports.len() as u8;
                CC | 16 << 8
            }
            GET_CONNECTOR_CAPABILITY if (1..=self.ports.len()).contains(&n) => {
                self.message[..2].copy_from_slice(&[0b100, 0b11]);
                CC | 2 << 8
            }
            GET_CONNECTOR_STATUS if self.refuse_status => CC | ERROR,
            GET_CONNECTOR_STATUS if (1..=self.ports.len()).contains(&n) => {
                self.message = self.ports[n - 1];
                self.ports[n - 1][0..2].copy_from_slice(&[0, 0]);
                CC | 9 << 8
            }
            GET_CONNECTOR_CAPABILITY | GET_CONNECTOR_STATUS => CC | ERROR,
            _ => CC | NOT_SUPPORTED,
        };
    }
}
