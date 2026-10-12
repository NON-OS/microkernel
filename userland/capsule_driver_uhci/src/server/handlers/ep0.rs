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
//! A control transfer on a slot's EP0, and the reply that hands back what
//! it moved: the length as a u16 in a head, then the bytes.

use crate::env::{Clock, Io, Mem};
use crate::sched::layout::CTRL_BUF;
use crate::sched::td::Target;
use crate::server::context::Ctx;
use crate::server::reply::{data, DATA_AT};
use crate::wire::{Request, E_INVAL, E_IO, E_PIPE};
use crate::xfer::control::{control as run, CtrlError};

/// Run `setup` on `slot`'s EP0; Ok is the data bytes moved.
pub fn ep0<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
    slot: u8,
    setup: [u8; 8],
) -> Result<usize, i32> {
    let dev = cx.table.dev(slot).filter(|d| d.port != 0).copied().ok_or(E_INVAL)?;
    let t = Target { addr: slot, ep: 0, low_speed: dev.low_speed };
    run(&cx.mem, &cx.clock, t, dev.mps0, setup).map_err(|e| match e {
        CtrlError::Stalled => E_PIPE,
        CtrlError::TooLong => E_INVAL,
        _ => E_IO,
    })
}

/// A reply of `n` bytes from the control buffer after a `prefix`-byte
/// head whose first two bytes are `n`.
pub fn give<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
    req: &Request,
    tx: &mut [u8],
    n: usize,
    prefix: usize,
) -> usize {
    tx[DATA_AT..DATA_AT + prefix].fill(0);
    tx[DATA_AT..DATA_AT + 2].copy_from_slice(&(n as u16).to_le_bytes());
    cx.mem.read_bytes(CTRL_BUF, &mut tx[DATA_AT + prefix..DATA_AT + prefix + n]);
    data(tx, req, prefix + n)
}
