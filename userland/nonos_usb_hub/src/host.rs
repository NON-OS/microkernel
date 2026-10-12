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
//! What the hub class needs of a USB host controller driver and of the
//! capsule running it. Failures are the host wire's statuses, so a refusal
//! (E_INVAL from a host that does not serve an op) can be told apart.

use crate::place::Place;
use crate::status_endpoint::StatusEndpoint;

/// The host wire's "not served or not valid" and "held by another driver".
pub const E_INVAL: i32 = -22;
pub const E_BUSY: i32 = -16;

/// A control request's setup stage (USB 2.0 section 9.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setup {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

pub trait Host {
    /// One control transfer on `slot`'s EP0; Ok is the data bytes moved,
    /// into `data` for an IN request.
    fn control(&mut self, slot: u8, setup: Setup, data: &mut [u8]) -> Result<usize, i32>;
    /// Tell the controller `slot` is a hub of `ports` ports whose TT
    /// thinks for `think` (wHubCharacteristics bits 6:5).
    fn configure_hub(&mut self, slot: u8, ports: u8, think: u8) -> Result<(), i32>;
    /// Claim the hub's status change endpoint.
    fn status_endpoint(&mut self, slot: u8, ep: StatusEndpoint) -> Result<(), i32>;
    fn enable_slot(&mut self) -> Result<u8, i32>;
    /// Address the device in `slot` where `place` says it sits.
    fn address_routed(&mut self, slot: u8, place: &Place) -> Result<(), i32>;
    fn disable_slot(&mut self, slot: u8);
    fn idle_ms(&mut self, ms: u64);
    fn now_ms(&mut self) -> i64;
    /// One finished log line, newline included.
    fn say(&mut self, line: &[u8]);
}
