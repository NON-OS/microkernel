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

//! Frames in: one look at the bulk IN pipe when the last transfer's walk
//! is done, then one frame a call from it (rx.rs). A transfer ends at a
//! short packet, which the bus reports as fewer bytes than were armed; one
//! that fills the buffer has not ended, so it and the rest of it up to its
//! end are let go, never walked.

use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::Bus;

use super::counts::bump;
use super::rx::{next, Next, DESC};
use super::state::Rtl;
use super::tx::recover;

fn take<B: Bus>(r: &mut Rtl<B>) -> Result<bool, i32> {
    let n = match r.bus.bulk_in_poll(&mut r.rx) {
        Ok(Some(n)) => n.min(r.rx.len()),
        Ok(None) => return Ok(false),
        Err(e) => return Err(recover(&mut r.bus, e, r.pipes.bulk_in)),
    };
    let (ended, was) = (n < r.rx.len(), r.overrun);
    r.overrun = !ended;
    if was || !ended {
        if !was {
            bump(&mut r.counts.overruns);
        }
        return Ok(false);
    }
    if (1..=DESC).contains(&n) {
        bump(&mut r.counts.runts);
    }
    (r.have, r.at) = (n, 0);
    Ok(true)
}

pub(super) fn recv<B: Bus>(r: &mut Rtl<B>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    if r.at >= r.have && !take(r)? {
        return Ok(None);
    }
    while r.at < r.have {
        match next(&r.rx[..r.have], r.at) {
            Next::Frame { start, len, next } => {
                r.at = next;
                if len > ETH_FRAME_MAX || len > out.len() {
                    bump(&mut r.counts.too_long);
                    continue;
                }
                out[..len].copy_from_slice(&r.rx[start..start + len]);
                return Ok(Some(len));
            }
            Next::Broken => bump(&mut r.counts.broken),
            Next::End => {}
        }
        break;
    }
    r.at = r.have;
    Ok(None)
}
