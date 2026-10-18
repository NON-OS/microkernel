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

//! The kernel's side of the boot framebuffer (lead L31) as a model: the
//! record it lists, a claim only driver.bootfb0's server gets, the mode for
//! the holder only, and bar 0 as host memory. Every call is written down,
//! so a test sees the order bootfb asked in.

use crate::firmware::{Firmware, Why};
use crate::geometry::INFO_LEN;

pub const EPOCH: u64 = 7;

pub struct Model {
    pub bar: Vec<u32>,
    pub info: [u8; INFO_LEN],
    pub calls: Vec<&'static str>,
    pub said: Vec<String>,
    pub announced: bool,
    pub holder: bool,
    /// A GPU driver has the scanout: every claim refused for good (L79).
    pub taken: bool,
}

impl Firmware for Model {
    fn announce(&mut self) -> Result<(), Why> {
        self.calls.push("announce");
        self.announced = true;
        Ok(())
    }
    fn claim(&mut self, id: u64) -> Result<u64, Why> {
        self.calls.push("claim");
        if self.taken {
            return Err(Why::Refused);
        }
        // EBUSY for anyone but driver.bootfb0's server.
        self.holder = self.announced && id == 42;
        self.holder.then_some(EPOCH).ok_or(Why::Claim)
    }
    fn geometry(&mut self, _id: u64) -> Result<[u8; INFO_LEN], Why> {
        self.calls.push("geometry");
        self.holder.then_some(self.info).ok_or(Why::Geometry)
    }
    fn map(&mut self, _id: u64, epoch: u64, size: u64) -> Result<*mut u8, Why> {
        self.calls.push("map");
        let whole = size as usize <= self.bar.len() * 4;
        (epoch == EPOCH && whole).then_some(self.bar.as_mut_ptr() as *mut u8).ok_or(Why::Map)
    }
    fn release(&mut self, _id: u64) {
        self.calls.push("release");
        self.holder = false;
    }
    fn say(&mut self, line: &str) {
        self.said.push(line.into());
    }
}
