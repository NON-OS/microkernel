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

//! A bound RNDIS device as the frame service sees it: frames each way,
//! and the link as the device last reported it (link_watch.rs).

use nonos_usbnet::{Bus, Nic};

use super::link::Rndis;
use super::link_watch::look_at_link;
use super::transfer::{recv, send};

impl<B: Bus> Nic for Rndis<B> {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// Up from bind until an INDICATE_STATUS says MEDIA_DISCONNECT.
    fn link_up(&self) -> bool {
        self.link
    }

    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        send(self, frame)
    }

    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        look_at_link(self);
        recv(self, out)
    }
}
