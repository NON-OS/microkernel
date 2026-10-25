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

//! A host whose every port GET_STATUS answers `n` of four given bytes.
//! The port read asks nothing else, so the other requests are unreachable.

use nonos_usb_hub::{Host, Place, Setup, StatusEndpoint};

pub struct Answers {
    pub bytes: [u8; 4],
    pub n: usize,
}

impl Host for Answers {
    fn control(&mut self, _slot: u8, s: Setup, data: &mut [u8]) -> Result<usize, i32> {
        assert_eq!((s.request_type, s.request, s.length), (0xA3, 0x00, 4));
        data[..self.n].copy_from_slice(&self.bytes[..self.n]);
        Ok(self.n)
    }
    fn configure_hub(&mut self, _: u8, _: u8, _: u8) -> Result<(), i32> {
        unreachable!("the port read configures nothing")
    }
    fn status_endpoint(&mut self, _: u8, _: StatusEndpoint) -> Result<(), i32> {
        unreachable!("the port read claims nothing")
    }
    fn enable_slot(&mut self) -> Result<u8, i32> {
        unreachable!("the port read enables no slot")
    }
    fn address_routed(&mut self, _: u8, _: &Place) -> Result<(), i32> {
        unreachable!("the port read addresses nothing")
    }
    fn disable_slot(&mut self, _: u8) {
        unreachable!("the port read disables no slot")
    }
    fn park(&mut self, _: u8, _: &Place, _: u16) -> Result<(), i32> {
        unreachable!("the port read parks nothing")
    }
    fn idle_ms(&mut self, _: u64) {}
    fn now_ms(&mut self) -> i64 {
        0
    }
    fn say(&mut self, _: &[u8]) {}
}
