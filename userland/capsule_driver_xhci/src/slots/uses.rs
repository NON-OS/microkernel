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
//! What a request on a slot touches: its default control pipe, or one of
//! its pipes configured or moved on. A shared slot's pipes are policed by
//! who configured them (owners.rs); a request that names no slot is not.

use super::pairs::pair_of;
use crate::protocol::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pipe {
    Interrupt(u8),
    /// A bulk pair: 0 the first, 1 the second.
    Bulk(u8),
    IsochOut,
    IsochIn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Use {
    /// Endpoint 0: any request at all, SET_CONFIGURATION among them.
    Control,
    Configure(Pipe),
    Transfer(Pipe),
}

/// The slot `op` with `body` acts on and how; `None` for a request that
/// acts on no addressed device's pipe (port, slot and hub management), or
/// only reads its device or first configuration descriptor, which a driver
/// deciding whether to join a shared device must read before it claims.
pub fn use_of(op: u16, body: &[u8]) -> Option<(u8, Use)> {
    let (&slot, b1) = (body.first()?, body.get(1).copied());
    let isoch = |ep: u8| if ep & 0x80 != 0 { Pipe::IsochIn } else { Pipe::IsochOut };
    let pipe = match op {
        OP_CONTROL_TRANSFER => return Some((slot, Use::Control)),
        OP_ALLOC_TRANSFER_RING => Use::Configure(Pipe::Interrupt(dci_of(b1?))),
        OP_INTERRUPT_IN | OP_INTERRUPT_OUT => Use::Transfer(Pipe::Interrupt(b1?)),
        OP_CONFIGURE_BULK => Use::Configure(Pipe::Bulk(0)),
        OP_CONFIGURE_BULK2 => Use::Configure(Pipe::Bulk(1)),
        OP_CONFIGURE_STREAMS | OP_BULK_OUT | OP_BULK_IN | OP_BULK_IN_POLL => {
            Use::Transfer(Pipe::Bulk(pair_of(b1? & 0x0F)? as u8))
        }
        OP_RESET_BULK => Use::Transfer(Pipe::Bulk(b1? >> 1 & 1)),
        OP_CONFIGURE_ISOCH => Use::Configure(isoch(b1?)),
        OP_ISOCH_OUT => Use::Transfer(Pipe::IsochOut),
        OP_ISOCH_IN => Use::Transfer(Pipe::IsochIn),
        _ => return None,
    };
    Some((slot, pipe))
}

fn dci_of(ep_address: u8) -> u8 {
    super::dci::dci_from_ep_address(ep_address)
}
