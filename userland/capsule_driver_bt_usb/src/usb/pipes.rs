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

//! The HCI transport's USB pipes, backed by driver.xhci0: commands on the
//! default endpoint, events from the interrupt endpoint, ACL data on the
//! bulk pair (Vol 4, Part B, 2.1).

use super::ops::interrupt_poll;
use crate::run::Desk;
use nonos_bt_hci::usb::{Setup, UsbPipes};
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::Bus;

pub struct XhciPipes {
    pub bus: XhciBus,
    /// The event endpoint's device context index.
    pub dci: u8,
    /// The octets one look at it arms (`limit::read_limit`).
    pub event_read: usize,
    /// Every pause answers NBTC requests (`run/desk.rs`).
    pub desk: Desk,
}

impl UsbPipes for XhciPipes {
    fn control_out(&mut self, s: Setup, data: &[u8]) -> Result<(), i32> {
        let setup = nonos_usbnet::Setup::new(s.request_type, s.request, s.value, s.index);
        self.bus.control_out(setup, data)
    }
    fn interrupt_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        // Never more than driver.xhci0 reads, whatever the transport asks.
        let n = out.len().min(self.event_read);
        interrupt_poll(self.bus.xhci, self.bus.slot, self.dci, &mut out[..n])
    }
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32> {
        self.bus.bulk_out(data)
    }
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        self.bus.bulk_in_poll(out)
    }
    /// About a millisecond: what the HCI transport asks for between two
    /// empty looks at the controller.
    fn pause(&mut self) {
        self.desk.wait(1);
    }
}
