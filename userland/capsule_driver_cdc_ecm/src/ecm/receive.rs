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

//! One frame in over the bulk IN pipe. A transfer ends at a short packet
//! or a zero-length one, which the bus reports as fewer bytes than were
//! armed; one that fills the buffer has not ended, so it and the rest of it
//! up to its end are let go, never read past the buffer or taken as frames.

use nonos_usbnet::nic::ETH_HEADER;
use nonos_usbnet::Bus;

use super::counts::bump;
use super::frame::received_len;
use super::link::Ecm;
use super::transfer::recover;

pub(super) fn recv<B: Bus>(ecm: &mut Ecm<B>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    let n = match ecm.bus.bulk_in_poll(&mut ecm.rx) {
        Ok(Some(n)) => n.min(ecm.rx.len()),
        Ok(None) => return Ok(None),
        Err(e) => return Err(recover(&mut ecm.bus, e, ecm.pipes.bulk_in)),
    };
    let (ended, was) = (n < ecm.rx.len(), ecm.overrun);
    ecm.overrun = !ended;
    let c = &mut ecm.counts;
    if was || !ended {
        if !was {
            bump(&mut c.overruns);
        }
        return Ok(None);
    }
    let Some(len) = received_len(n) else {
        bump(if n < ETH_HEADER { &mut c.runts } else { &mut c.cut });
        return Ok(None);
    };
    if len > out.len() {
        bump(&mut c.too_long);
        return Ok(None);
    }
    out[..len].copy_from_slice(&ecm.rx[..len]);
    Ok(Some(len))
}
