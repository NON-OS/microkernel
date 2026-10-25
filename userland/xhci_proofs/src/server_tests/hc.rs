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

//! A host controller behind the server, written from xHCI 1.2: what it
//! holds. It runs the command ring (`hc_command`) and each slot's EP0 ring
//! (`hc_ep0`) when their doorbells are rung (`hc_device`), and posts every
//! completion through the event ring producer.

use super::hc_ep0::Setup;
use super::hc_input::Ran;
use super::hc_race::Race;
use super::hc_reset::Resets;
use crate::event_ring::producer::Producer;

pub const SLOTS: u8 = 8;

pub struct Hc {
    pub producer: Producer,
    pub ring: u64,
    pub cycle: bool,
    pub enabled: [bool; SLOTS as usize + 1],
    pub ep0: [(u64, bool); SLOTS as usize + 1],
    /// The slot whose next Address Device never completes (SET_ADDRESS NAKed).
    pub stall_address: Option<u8>,
    /// The command the ring is stuck on, and its slot.
    pub stuck: Option<(u64, u8)>,
    pub ran: Vec<Ran>,
    /// What the device sends for each IN data stage.
    pub answer: fn(&Setup) -> Vec<u8>,
    /// Every IN data stage STALLs, halting EP0.
    pub stall_in: bool,
    /// Reset, Stop Endpoint and Set TR Dequeue fail with a TRB Error.
    pub refuse_recovery: bool,
    pub race: Race,
    /// USBSTS shows a Host System Error, cleared only if resets are followed.
    pub fault: bool,
    /// How an HCRST is taken (`hc_reset`).
    pub resets: Resets,
    pub bulk: super::hc_bulk::Bulk,
}

impl Hc {
    pub fn new(producer: Producer) -> Self {
        Self {
            producer,
            ring: 0,
            cycle: true,
            enabled: [false; SLOTS as usize + 1],
            ep0: [(0, true); SLOTS as usize + 1],
            stall_address: None,
            stuck: None,
            ran: Vec::new(),
            answer: |_| Vec::new(),
            stall_in: false,
            refuse_recovery: false,
            race: Race::default(),
            fault: false,
            resets: Resets::Ignored,
            bulk: Default::default(),
        }
    }
}
