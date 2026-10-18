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

//! The USB transfers the HCI transport makes, for a capsule to back with
//! its USB controller driver once the device is addressed, configured and
//! its three endpoints are set up. Errors are that driver's errnos.

/// A setup packet without its wLength, which is the data stage's length
/// (USB 2.0, 9.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setup {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
}

/// The request that carries one HCI command: a class request to the
/// device, host to device, bRequest 0, wValue 0, wIndex 0, with the
/// command packet as its data stage (Vol 4, Part B, 2.2).
pub const COMMAND: Setup = Setup { request_type: 0x20, request: 0, value: 0, index: 0 };

pub trait UsbPipes {
    /// A control transfer on the default endpoint with `data` as its data
    /// stage out.
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32>;
    /// One look at the interrupt IN endpoint with `out.len()` bytes armed:
    /// `None` while the device has sent nothing.
    fn interrupt_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32>;
    /// Send `data` on the bulk OUT endpoint; the bytes the device took.
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32>;
    /// One look at the bulk IN endpoint, as `interrupt_in_poll`.
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32>;
    /// Wait about a millisecond.
    fn pause(&mut self);
}
