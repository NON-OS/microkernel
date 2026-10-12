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

use nonos_usbnet::{Bus, Nic, Taken};

use super::link::Rtl8153;
use super::transfer::{recv, send};
use super::watch::watch;

impl<B: Bus> Nic for Rtl8153<B> {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// Written to PLA_IDR at bind, or the bind failed by name.
    fn taken(&self) -> Taken {
        Taken::Programmed
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

    /// The link read on the run loop's clock, after each answer: net.core
    /// asks a NIC only for its link until that reads up, so a link read
    /// inside `recv` would never be read at all.
    fn tick(&mut self) -> Result<(), i32> {
        watch(self)
    }
}
