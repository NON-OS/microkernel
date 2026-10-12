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

//! A bound NCM device as the frame service sees it.

use nonos_usbnet::{Bus, Nic, Taken};

use super::link::Ncm;
use super::transfer::{recv, send};

impl<B: Bus> Nic for Ncm<B> {
    fn mac(&self) -> [u8; 6] {
        self.station.0
    }

    fn taken(&self) -> Taken {
        self.station.1
    }

    /// The link is the device being bound: NETWORK_CONNECTION arrives on an
    /// interrupt endpoint driver.xhci0 does not configure beside bulk pipes.
    fn link_up(&self) -> bool {
        true
    }

    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        send(self, frame)
    }

    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        recv(self, out)
    }
}
