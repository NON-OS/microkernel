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

//! Driving the host from the peripheral's side.

use super::run::Run;
use nonos_bt_hogp::host::Battery;
use nonos_bt_hogp::report::Input;

impl Run {
    /// One PDU from the peripheral to the host, 10 ms after the last.
    pub fn deliver(&mut self, pdu: &[u8]) {
        self.now += 10;
        let step = self.host.on_pdu(pdu, self.now);
        self.take(step);
    }

    /// A notification of `value` on `handle`; the events it made.
    pub fn notify(&mut self, handle: u16, value: &[u8]) -> Vec<Input> {
        let before = self.input.len();
        self.deliver(&[&[0x1b][..], &handle.to_le_bytes(), value].concat());
        self.input[before..].to_vec()
    }

    /// A notification of a Battery Level; the level it gave, if any.
    pub fn notify_level(&mut self, handle: u16, value: &[u8]) -> Option<Battery> {
        let before = self.batteries.len();
        self.notify(handle, value);
        self.batteries.get(before).copied()
    }
}
