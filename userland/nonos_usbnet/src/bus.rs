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

//! What a bound USB network device is to its driver: control requests on
//! endpoint 0, and one bulk IN and one bulk OUT pipe. driver.xhci0 is the
//! real one (`XhciBus`); the proofs script a device behind the same trait.

use crate::setup::Setup;

/// The bulk pipes of the data interface, as its endpoint descriptors and
/// SuperSpeed companions give them.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Pipes {
    pub bulk_in: u8,
    pub bulk_out: u8,
    pub max_packet_in: u16,
    pub max_packet_out: u16,
    pub burst_in: u8,
    pub burst_out: u8,
}

pub trait Bus {
    /// A request with a data stage in; the bytes that came.
    fn control_in(&mut self, setup: Setup, out: &mut [u8]) -> Result<usize, i32>;
    /// A request with `data` as its data stage, or none when it is empty.
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32>;
    fn configure_bulk(&mut self, pipes: &Pipes) -> Result<(), i32>;
    /// Send 1 to BULK_MAX bytes; the bytes the device took.
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32>;
    /// One look at the IN pipe, with `out.len()` bytes armed: `None` while
    /// the device has sent nothing.
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32>;
    fn reset_bulk(&mut self, dir_in: bool) -> Result<(), i32>;
}
