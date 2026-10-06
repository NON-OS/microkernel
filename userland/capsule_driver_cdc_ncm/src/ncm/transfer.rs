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

//! One NTB out per frame, and frames in from the datagrams of each NTB
//! received. A stalled pipe is brought back before the error goes up, so
//! the next transfer finds it working.

use nonos_usbnet::halt::clear_halt;
use nonos_usbnet::xhci::{E_INVAL, E_IO, E_PIPE};
use nonos_usbnet::Bus;

use super::link::Ncm;
use super::ntb_in::datagrams;
use super::ntb_out::build;

pub(super) fn send<B: Bus>(ncm: &mut Ncm<B>, frame: &[u8]) -> Result<(), i32> {
    let Some(n) = build(&mut ncm.tx, frame, ncm.seq, &ncm.shape) else { return Err(E_INVAL) };
    ncm.seq = ncm.seq.wrapping_add(1);
    match ncm.bus.bulk_out(&ncm.tx[..n]) {
        Ok(m) if m == n => Ok(()),
        Ok(_) => Err(E_IO),
        Err(e) => Err(recover(&mut ncm.bus, e, ncm.pipes.bulk_out)),
    }
}

/// The next datagram that fits `out`; the IN pipe is polled only once the
/// last block's datagrams are all handed up.
pub(super) fn recv<B: Bus>(ncm: &mut Ncm<B>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    if ncm.pending.is_empty() {
        match ncm.bus.bulk_in_poll(&mut ncm.rx) {
            Ok(Some(n)) => datagrams(&ncm.rx[..n], ncm.rx_max, &mut ncm.pending),
            Ok(None) => return Ok(None),
            Err(e) => return Err(recover(&mut ncm.bus, e, ncm.pipes.bulk_in)),
        }
    }
    // A datagram longer than the stack's frame is dropped, as one past the
    // MTU would be.
    while let Some((at, len)) = ncm.pending.pop_front() {
        if len <= out.len() {
            out[..len].copy_from_slice(&ncm.rx[at..at + len]);
            return Ok(Some(len));
        }
    }
    Ok(None)
}

fn recover<B: Bus>(bus: &mut B, e: i32, endpoint: u8) -> i32 {
    if e == E_PIPE {
        let _ = clear_halt(bus, endpoint);
    }
    e
}
