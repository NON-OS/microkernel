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

//! Frames over the bulk pipes: one frame out per transfer, and the frames
//! of each bulk IN queued and handed up one at a time. A stalled pipe is
//! brought back before the error goes up, as in the ECM driver.

use nonos_usbnet::halt::clear_halt;
use nonos_usbnet::xhci::{E_INVAL, E_IO, E_PIPE};
use nonos_usbnet::Bus;

use super::link::Rtl8153;
use super::rx::frames;
use super::tx::put_frame;
use super::watch::watch;

/// Frames kept from bulk INs not yet handed up; past it they are dropped.
const QUEUE_MAX: usize = 32;
/// VLAN_ETH_FRAME_LEN: a full frame with its tag left in (fifo.rs).
const FRAME_MAX: usize = 1518;

pub(super) fn send<B: Bus>(nic: &mut Rtl8153<B>, frame: &[u8]) -> Result<(), i32> {
    let n = put_frame(frame, &mut nic.tx).ok_or(E_INVAL)?;
    match nic.dev.bus.bulk_out(&nic.tx[..n]) {
        Ok(m) if m == n => Ok(()),
        Ok(_) => Err(E_IO),
        Err(e) => Err(recover(&mut nic.dev.bus, e, nic.pipes.bulk_out)),
    }
}

pub(super) fn recv<B: Bus>(nic: &mut Rtl8153<B>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    watch(nic)?;
    if nic.queue.is_empty() {
        let n = match nic.dev.bus.bulk_in_poll(&mut nic.rx) {
            Ok(Some(n)) => n.min(nic.rx.len()),
            Ok(None) => return Ok(None),
            Err(e) => return Err(recover(&mut nic.dev.bus, e, nic.pipes.bulk_in)),
        };
        let queue = &mut nic.queue;
        frames(&nic.rx[..n], |f| {
            if queue.len() < QUEUE_MAX && f.len() <= FRAME_MAX {
                queue.push_back(f.to_vec());
            }
        });
    }
    // A frame larger than the caller's buffer is dropped, not cut.
    let Some(frame) = nic.queue.pop_front().filter(|f| f.len() <= out.len()) else {
        return Ok(None);
    };
    out[..frame.len()].copy_from_slice(&frame);
    Ok(Some(frame.len()))
}

fn recover<B: Bus>(bus: &mut B, e: i32, endpoint: u8) -> i32 {
    if e == E_PIPE {
        let _ = clear_halt(bus, endpoint);
    }
    e
}
