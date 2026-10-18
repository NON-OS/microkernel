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

//! The device's registers and what it has done.

use super::events::{Event, Faults};
use super::memory::Memory;
use super::ring::Ring;

#[derive(Default)]
pub struct Device {
    pub mem: Memory,
    pub faults: Faults,
    pub log: Vec<Event>,
    pub sent: Vec<Vec<u8>>,
    pub vrrs: u32,
    pub uvrs: u32,
    pub dsa: u64,
    pub mac: [u8; 6],
    pub answer: u32,
    pub active: bool,
    pub link: bool,
    pub masked: bool,
    pub mtu: u32,
    pub rx_mode: u32,
    pub vf0: u32,
    /// Why the last ACTIVATE_DEV was refused.
    pub refusal: Option<&'static str>,
    pub tx: Ring,
    pub tx_comp: Ring,
    pub rx: Ring,
    pub rx1: Ring,
    pub rx_comp: Ring,
}

impl Device {
    pub fn new(mem: Memory) -> Self {
        Self { mem, link: true, ..Self::default() }
    }

    pub(super) fn note(&mut self, e: Event) {
        self.log.push(e);
    }
}
