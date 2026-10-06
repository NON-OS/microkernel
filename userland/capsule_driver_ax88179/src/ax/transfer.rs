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

//! Frames over the bulk pipes: one frame per bulk OUT, and the frames of a
//! bulk IN queued and handed up one per receive. The pipe is polled only
//! when the queue is empty. A stalled pipe is brought back before the
//! error goes up, so the next transfer finds it working.

use nonos_usbnet::halt::clear_halt;
use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::xhci::{E_INVAL, E_IO, E_PIPE};
use nonos_usbnet::Bus;

use super::link::Ax88179;
use super::rx::frames;
use super::tx::tx_transfer;

pub(super) fn send<B: Bus>(ax: &mut Ax88179<B>, frame: &[u8]) -> Result<(), i32> {
    if frame.len() > ETH_FRAME_MAX {
        return Err(E_INVAL);
    }
    let n = tx_transfer(frame, ax.pipes.max_packet_out, &mut ax.tx);
    match ax.bus.bulk_out(&ax.tx[..n]) {
        Ok(m) if m == n => Ok(()),
        Ok(_) => Err(E_IO),
        Err(e) => Err(recover(&mut ax.bus, e, ax.pipes.bulk_out)),
    }
}

pub(super) fn recv<B: Bus>(ax: &mut Ax88179<B>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    if ax.next >= ax.frames.len() {
        let n = match ax.bus.bulk_in_poll(&mut ax.rx) {
            Ok(Some(n)) => n.min(ax.rx.len()),
            Ok(None) => return Ok(None),
            Err(e) => return Err(recover(&mut ax.bus, e, ax.pipes.bulk_in)),
        };
        ax.next = 0;
        // A malformed transfer leaves the queue empty: it is dropped.
        frames(&ax.rx[..n], &mut ax.frames);
    }
    let Some(&(at, len)) = ax.frames.get(ax.next) else { return Ok(None) };
    ax.next += 1;
    if len > out.len() {
        return Ok(None);
    }
    out[..len].copy_from_slice(&ax.rx[at..at + len]);
    Ok(Some(len))
}

fn recover<B: Bus>(bus: &mut B, e: i32, endpoint: u8) -> i32 {
    if e == E_PIPE {
        let _ = clear_halt(bus, endpoint);
    }
    e
}
