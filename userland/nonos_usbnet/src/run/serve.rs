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
//! The driver's life: look for its device, serve the stack's NNET
//! requests, and when the device stops answering, give it back and look
//! again. Requests are answered while looking: the link reads down.

use alloc::vec;

use nonos_libc::Deadline;

use super::bind_fn::BindFn;
use super::me::own_pid;
use super::one::serve_one;
use crate::nic::{Nic, ETH_FRAME_MAX};
use crate::nnet::{Stats, DATA_AT, HDR_LEN};
use crate::say::{say, say_up};
use crate::scan::{Bound, NetScanner};
use crate::xhci::disable_slot;

/// How long a receive waits before the next pass while no device is bound.
const SEARCH_POLL_MS: u64 = 1_000;
/// Failures in a row, transfers and the driver's own looks alike, before
/// the device is taken as gone.
pub const LOST_AFTER: u32 = 8;

/// `service` is the name the capsule is spawned under, `driver.<x>0`.
pub fn run<N: Nic>(tag: &'static [u8], service: &str, bind: BindFn<N>) -> ! {
    let mut rx = vec![0u8; HDR_LEN + ETH_FRAME_MAX + 64];
    let mut tx = vec![0u8; DATA_AT + 4 + ETH_FRAME_MAX + 64];
    say(tag, &[b"started, looking for a device"]);
    let (mut stats, mut scanner) = (Stats::default(), NetScanner::new(tag));
    let mut bound: Option<Bound<N>> = None;
    let (me, mut next_pass) = (own_pid(service), Deadline::after_ms(0));
    loop {
        let wait = if bound.is_some() { 0 } else { SEARCH_POLL_MS };
        serve_one(&mut rx, &mut tx, wait, me, bound.as_mut().map(|b| &mut b.nic), &mut stats);
        if let Some(b) = bound.as_mut() {
            stats.ticked(b.nic.tick());
        }
        if stats.failing >= LOST_AFTER {
            if let Some(b) = bound.take() {
                say(tag, &[b"device stopped answering, given back"]);
                disable_slot(b.xhci, b.slot);
                scanner.lost(b.xhci, b.port);
                (stats.losses, stats.failing) = (stats.losses.saturating_add(1), 0);
            }
        }
        // net.core asks for the link every second whether or not a device
        // is bound; the ports are looked at on the clock, not per request.
        if bound.is_none() && next_pass.expired() {
            next_pass = Deadline::after_ms(SEARCH_POLL_MS);
            bound = scanner.pass(tag, bind);
            if let Some(b) = &bound {
                say_up(tag, b.port, b.nic.taken());
                stats.binds = stats.binds.saturating_add(1);
            }
        }
    }
}
