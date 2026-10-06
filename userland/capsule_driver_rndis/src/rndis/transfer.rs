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

//! One frame out and one in over the bulk pipes. A bulk IN is read only
//! when every frame of the last one has gone up. A stalled pipe is brought
//! back before the error goes up, so the next transfer finds it working.

use nonos_usbnet::halt::clear_halt;
use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};
use nonos_usbnet::xhci::{E_INVAL, E_IO, E_PIPE};
use nonos_usbnet::Bus;

use super::batch::frames;
use super::link::Rndis;
use super::packet::wrap;

pub(super) fn send<B: Bus>(nic: &mut Rndis<B>, frame: &[u8]) -> Result<(), i32> {
    if !(ETH_HEADER..=ETH_FRAME_MAX).contains(&frame.len()) {
        return Err(E_INVAL);
    }
    let n = wrap(frame, nic.pipes.max_packet_out, &mut nic.tx);
    // The bind took only devices whose MaxTransferSize holds this.
    if n > nic.limits.max_transfer as usize {
        return Err(E_INVAL);
    }
    match nic.bus.bulk_out(&nic.tx[..n]) {
        Ok(m) if m == n => Ok(()),
        Ok(_) => Err(E_IO),
        Err(e) => Err(recover(&mut nic.bus, e, nic.pipes.bulk_out)),
    }
}

pub(super) fn recv<B: Bus>(nic: &mut Rndis<B>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    if nic.queue.is_empty() {
        let n = match nic.bus.bulk_in_poll(&mut nic.rx) {
            Ok(Some(n)) => n.min(nic.rx.len()),
            Ok(None) => return Ok(None),
            Err(e) => return Err(recover(&mut nic.bus, e, nic.pipes.bulk_in)),
        };
        frames(&nic.rx[..n], &mut nic.queue);
    }
    // A frame larger than the stack's buffer is dropped, never cut.
    let Some(r) = nic.queue.pop_front().filter(|r| r.len() <= out.len()) else { return Ok(None) };
    out[..r.len()].copy_from_slice(&nic.rx[r.clone()]);
    Ok(Some(r.len()))
}

fn recover<B: Bus>(bus: &mut B, e: i32, endpoint: u8) -> i32 {
    if e == E_PIPE {
        let _ = clear_halt(bus, endpoint);
    }
    e
}
