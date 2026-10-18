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

//! The simulated controller as the USB pipes the transport drives.

use super::controller::respond;
use super::pipes::{give, SimPipes};
use nonos_bt_hci::usb::{Setup, UsbPipes};

impl UsbPipes for SimPipes {
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32> {
        self.commands.push((setup, data.to_vec(), self.interrupt.len()));
        for e in respond(&self.cfg, data) {
            self.send_event(&e);
        }
        Ok(())
    }
    fn interrupt_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        give(&mut self.interrupt, out)
    }
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32> {
        self.bulk_out.push(data.to_vec());
        Ok(self.bulk_take.unwrap_or(data.len()))
    }
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        give(&mut self.bulk_in, out)
    }
    fn pause(&mut self) {
        self.pauses += 1;
    }
}
