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
//! The running stream: batches collected from the endpoint into frames, a
//! frame read in pieces, and the stream stopped, its bandwidth given back
//! and the compositor told.

use nonos_uvc::Payload;

use crate::batch::payloads;
use crate::device::Dev;
use crate::isoch::{isoch_in, per_batch, DATA_AT, REPLY_LEN};
use crate::wire::{reply, Request, CHUNK, E_AGAIN};

/// Batches in a row in error before the stream is stopped.
const LOSSES: u8 = 3;

/// One batch queued and the one before collected; whether it went well.
pub fn pump(d: &mut Dev, resp: &mut [u8; REPLY_LEN]) -> bool {
    let (Some(bus), Some((s, asm))) = (d.bus(), d.stream.as_mut()) else { return true };
    let Ok(n) = isoch_in(bus.xhci, bus.slot, per_batch(&s.alt), resp) else {
        d.losses += 1;
        if d.losses >= LOSSES {
            stop(d);
        }
        return false;
    };
    d.losses = 0;
    for p in payloads(&resp[DATA_AT..DATA_AT + n.min(REPLY_LEN - DATA_AT)]) {
        if let Payload::Frame = asm.push(p) {
            d.session.sequence = d.session.sequence.wrapping_add(1);
        }
    }
    true
}

pub fn read_frame(d: &Dev, req: &Request, offset: u32, tx: &mut [u8]) -> usize {
    let Some((_, asm)) = d.stream.as_ref().filter(|_| d.session.sequence > 0) else {
        return reply(tx, req, E_AGAIN, &[]);
    };
    let f = asm.frame();
    let from = (offset as usize).min(f.len());
    let piece = &f[from..(from + CHUNK).min(f.len())];
    let mut out = alloc::vec::Vec::with_capacity(12 + piece.len());
    out.extend(d.session.sequence.to_le_bytes());
    out.extend((f.len() as u32).to_le_bytes());
    out.extend((from as u32).to_le_bytes());
    out.extend_from_slice(piece);
    reply(tx, req, 0, &out)
}

pub fn stop(d: &mut Dev) {
    if let (Some(mut bus), Some(b)) = (d.bus(), d.bound.as_ref()) {
        let _ = crate::start::stop(&mut bus, &b.nic);
    }
    if d.stream.take().is_some() {
        crate::notify::capture(false);
    }
    d.session.stop();
    d.losses = 0;
}
