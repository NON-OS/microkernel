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

//! The adapter as the NNET frame service drives it (nonos_usbnet's `Nic`):
//! the station address settled before the link was first reported up, the
//! host's word on the link, frames out through the send buffer and frames
//! in from the receive buffer.

use nonos_usbnet::station::Taken;
use nonos_usbnet::Nic;
use nonos_vmbus::Pages;

use crate::constants::timing::SEND_MS;
use crate::host::Kernel;
use crate::link::{Link, LinkError};

pub type NetLink = Link<Kernel, Pages>;

/// errno values the frame service passes on.
const E_IO: i32 = -5;
const E_AGAIN: i32 = -11;

pub struct Net {
    pub link: NetLink,
    pub mac: [u8; 6],
    /// The link at the start, before the host said otherwise.
    pub started_up: bool,
}

fn errno(e: LinkError) -> i32 {
    match e {
        LinkError::Busy | LinkError::Silent => E_AGAIN,
        LinkError::Broken | LinkError::TooBig => E_IO,
    }
}

impl Nic for Net {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    fn taken(&self) -> Taken {
        // The host carries frames for this address: the drawn one it took,
        // or its own.
        Taken::Programmed
    }

    fn link_up(&self) -> bool {
        !self.link.dead && self.link.link.unwrap_or(self.started_up)
    }

    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        self.link.send_frame(frame, SEND_MS).map_err(errno)
    }

    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        if self.link.frames.is_empty() {
            self.link.service().map_err(errno)?;
        }
        let Some(f) = self.link.frames.pop_front() else { return Ok(None) };
        let n = f.len().min(out.len());
        out[..n].copy_from_slice(&f[..n]);
        Ok(Some(n))
    }

    fn tick(&mut self) -> Result<(), i32> {
        self.link.service().map_err(errno)
    }
}
