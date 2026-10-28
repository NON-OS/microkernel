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

//! Frames in: a bulk IN is taken only when every frame of the last one has
//! gone up, parsed once (rx_header.rs), and handed up a frame a request. A
//! transfer that fills the buffer has not ended, so it and the rest of it
//! up to its end are let go, never parsed: its trailer is not in it.

use nonos_usbnet::Bus;

use super::counts::bump;
use super::link::{Ax, SPANS_MAX};
use super::rx_header::parse;
use super::transfer::recover;

pub(super) fn recv<B: Bus>(ax: &mut Ax<B>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    if ax.next >= ax.spans.len() {
        fill(ax)?;
    }
    let Some(s) = ax.spans.get(ax.next).copied() else { return Ok(None) };
    ax.next += 1;
    let (at, len) = (s.at as usize, s.len as usize);
    if len > out.len() {
        bump(&mut ax.counts.too_long, 1);
        return Ok(None);
    }
    out[..len].copy_from_slice(&ax.rx[at..at + len]);
    Ok(Some(len))
}

fn fill<B: Bus>(ax: &mut Ax<B>) -> Result<(), i32> {
    ax.spans.clear();
    ax.next = 0;
    let n = match ax.bus.bulk_in_poll(&mut ax.rx) {
        Ok(Some(n)) => n.min(ax.rx.len()),
        Ok(None) => return Ok(()),
        Err(e) => return Err(recover(&mut ax.bus, e, ax.pipes.bulk_in)),
    };
    let (ended, was) = (n < ax.rx.len(), ax.overrun);
    ax.overrun = !ended;
    if was || !ended {
        if !was {
            bump(&mut ax.counts.overruns, 1);
        }
        return Ok(());
    }
    let tally = parse(&ax.rx[..n], &mut ax.spans, SPANS_MAX);
    ax.counts.add(tally);
    Ok(())
}
