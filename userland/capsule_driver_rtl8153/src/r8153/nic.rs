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

//! A bound RTL8153 as the frame service sees it.

use nonos_usbnet::{Bus, Nic};

use super::link::Rtl8153;
use super::transfer::{recv, send};

impl<B: Bus> Nic for Rtl8153<B> {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// The link as last read, at most a second old: answering never
    /// waits on the chip.
    fn link_up(&self) -> bool {
        self.link
    }

    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        send(self, frame)
    }

    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        recv(self, out)
    }
}
