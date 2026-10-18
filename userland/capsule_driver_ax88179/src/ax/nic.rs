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

//! A bound AX88179 as the frame service sees it.

use nonos_usbnet::{Bus, Nic, Taken};

use super::link::Ax88179;
use super::link_poll::poll_link;
use super::transfer::{recv, send};

impl<B: Bus> Nic for Ax88179<B> {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// Written to AX_NODE_ID at bind, or the bind failed by name.
    fn taken(&self) -> Taken {
        Taken::Programmed
    }

    fn link_up(&self) -> bool {
        self.link.is_some()
    }

    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        send(self, frame)
    }

    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        recv(self, out)
    }

    fn tick(&mut self) -> Result<(), i32> {
        poll_link(self)
    }
}
