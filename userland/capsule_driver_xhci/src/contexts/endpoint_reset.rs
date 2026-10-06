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

//! The input context that resets one endpoint's data toggle or sequence
//! number on the host side: Configure Endpoint with the endpoint both
//! dropped and added (xHCI 1.2 section 4.6.6, Linux `xhci_endpoint_reset`).
//! Reset Endpoint does this only for a Halted endpoint. A bulk pipe the
//! class driver resets after a timeout is Running or Stopped, so it is
//! stopped instead, and the host kept its toggle while CLEAR_FEATURE
//! (ENDPOINT_HALT) set the device's back to DATA0 (sequence 0 on USB 3).
//! The next packet then went out with the toggle the device does not
//! expect, it was taken as a retry and dropped, and the BOT transport
//! never got back in step.
//!
//! The endpoint context is the one the controller holds in the output
//! context, so its type, packet size, burst and interval stay as they were;
//! only its state is cleared and its dequeue pointer set to where the next
//! TRB goes.

use super::slot_copy::{copy_slot_context, read_dw, write_dw};
use crate::dma::DmaRegion;

const INPUT_CONTROL_INDEX: usize = 0;
const ADD_SLOT_FLAG: u32 = 1;
const ENDPOINT_DWORDS: usize = 5;
const EP_STATE_MASK: u32 = 0x7;
const DCS: u32 = 1;

/// Where the endpoint's ring goes on from: the next TRB's bus address and
/// the ring's producer cycle state.
#[derive(Clone, Copy)]
pub struct Dequeue {
    pub phys: u64,
    pub cycle: bool,
}

/// Write into `input` the context that drops and adds endpoint `dci`, its
/// context copied from `output` with the state cleared and the dequeue
/// pointer at `dequeue`.
pub fn write_endpoint_reset_input(
    input: &DmaRegion,
    output: &DmaRegion,
    context_size: u8,
    dci: u8,
    dequeue: Dequeue,
) {
    input.zero();
    write_dw(input, context_size, INPUT_CONTROL_INDEX, 0, 1 << dci);
    write_dw(input, context_size, INPUT_CONTROL_INDEX, 1, ADD_SLOT_FLAG | (1 << dci));
    copy_slot_context(input, output, context_size, dci);
    let (from, to) = (dci as usize, dci as usize + 1);
    for dw in 0..ENDPOINT_DWORDS {
        let v = match dw {
            0 => read_dw(output, context_size, from, 0) & !EP_STATE_MASK,
            2 => (dequeue.phys as u32 & !0xF) | if dequeue.cycle { DCS } else { 0 },
            3 => (dequeue.phys >> 32) as u32,
            _ => read_dw(output, context_size, from, dw),
        };
        write_dw(input, context_size, to, dw, v);
    }
}
