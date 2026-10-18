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

//! Reading back the command ring the way the controller reads it, and the
//! endpoints recovery is run on.

use super::fixture::{Fixture, CONTEXT_SIZE, SLOT};
use crate::contexts::device_context_bytes;
use crate::controller::EndpointAt;
use crate::dma::DmaRegion;
use crate::rings::command::CommandRing;
use crate::rings::transfer::TransferRing;
use crate::trb::Trb;

pub const CONFIGURE_ENDPOINT: u32 = 12;
pub const RESET_ENDPOINT: u32 = 14;
pub const STOP_ENDPOINT: u32 = 15;
pub const SET_TR_DEQUEUE: u32 = 16;

pub fn ring_base(cmd: &CommandRing) -> u64 {
    cmd.crcr_value() & !0x3F
}

/// The command TRB the driver wrote at bus address `bus`.
pub fn command_at(bus: u64) -> Trb {
    let host = nonos_libc::dma_host(bus).expect("command ring mapped");
    // SAFETY: a live grant holds the whole TRB.
    unsafe { (host as *const Trb).read() }
}

/// A slot's output context as a controller that halted no endpoint left it.
pub fn quiet_context(fx: &Fixture) -> DmaRegion {
    let output = fx.pool.alloc(device_context_bytes(CONTEXT_SIZE)).expect("output context");
    output.zero();
    output
}

/// Endpoint `dci` of the fixture's slot on `ring`, its state in `output`.
pub fn endpoint<'a>(ring: &'a TransferRing, output: &'a DmaRegion, dci: u8) -> EndpointAt<'a> {
    EndpointAt { ring, output, context_size: CONTEXT_SIZE, slot: SLOT, dci }
}
