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

//! The request page: one coherent DMA page below 4 GiB, since the request
//! port takes a 32-bit address. Each request is written at its start, its
//! status preset to a failure, and its address written to the port; the
//! host has answered by the time that write returns, so the status and the
//! body are read straight back.

use core::ptr::{read_volatile, write_volatile};

use crate::constants::pci::REQUEST_PORT;
use crate::constants::req::{BODY_MAX, HEADER_BYTES, HEADER_VERSION, RC_UNANSWERED};
use crate::regs::Port;

pub struct Page {
    va: u64,
    dev: u32,
    port: Port,
}

impl Page {
    /// `va` maps a page whose bus address is `dev`, held for the driver's
    /// life.
    pub const fn new(va: u64, dev: u32, port: Port) -> Self {
        Self { va, dev, port }
    }

    fn put(&mut self, at: usize, v: u32) {
        // SAFETY: `at` is inside the header and body, which fit the page.
        unsafe { write_volatile((self.va as usize + at) as *mut u32, v.to_le()) };
    }

    fn get(&self, at: usize) -> u32 {
        // SAFETY: as for `put`.
        u32::from_le(unsafe { read_volatile((self.va as usize + at) as *const u32) })
    }

    /// Request `kind` with `body` (words); the body as the host left it, or
    /// the host's failure status.
    pub fn perform(&mut self, kind: u32, body: &[u32]) -> Result<[u32; BODY_MAX / 4], i32> {
        let words = body.len().min(BODY_MAX / 4);
        let header = [(HEADER_BYTES + words * 4) as u32, HEADER_VERSION, kind];
        header.iter().enumerate().for_each(|(i, &w)| self.put(4 * i, w));
        self.put(12, RC_UNANSWERED as u32);
        self.put(16, 0);
        self.put(20, 0);
        body[..words].iter().enumerate().for_each(|(i, &w)| self.put(HEADER_BYTES + 4 * i, w));
        self.port.write32(REQUEST_PORT, self.dev);
        let rc = self.get(12) as i32;
        if rc < 0 {
            return Err(rc);
        }
        Ok(core::array::from_fn(|i| if i < words { self.get(HEADER_BYTES + 4 * i) } else { 0 }))
    }
}
