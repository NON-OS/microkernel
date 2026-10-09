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

//! The `Bus` of a device addressed on driver.xhci0.

use super::{bulk, control};
use crate::bus::{Bus, Pipes};
use crate::setup::Setup;

#[derive(Clone, Copy)]
pub struct XhciBus {
    pub xhci: u32,
    pub slot: u8,
}

impl Bus for XhciBus {
    fn control_in(&mut self, setup: Setup, out: &mut [u8]) -> Result<usize, i32> {
        control::control_in(self.xhci, self.slot, setup, out)
    }
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32> {
        control::control_out(self.xhci, self.slot, setup, data)
    }
    fn configure_bulk(&mut self, pipes: &Pipes) -> Result<(), i32> {
        bulk::configure_bulk(self.xhci, self.slot, pipes)
    }
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32> {
        bulk::bulk_out(self.xhci, self.slot, data)
    }
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        bulk::bulk_in_poll(self.xhci, self.slot, out)
    }
    fn reset_bulk(&mut self, dir_in: bool) -> Result<(), i32> {
        bulk::reset_bulk(self.xhci, self.slot, dir_in)
    }
}
