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

//! check.rs: what a written reset came to. The function must answer as
//! itself, not with a retry status, and its marker must read clear (undo
//! check 9 skips that read); then what the reset cleared is put back, the
//! decoders last. A marker that survived goes back as it was.

use super::bits_rule::{devctl_after, not_ready, BME, INTX_DISABLE, IO, MEM};
use super::end_rule::{checked, End, Next};
use super::marker_rule::marker_bit;
use super::method_rule::fallback;
use super::model_reset::write;
use super::world::World;
use super::world_cases::FALLBACK;
use super::world_knob::Knob;
use super::world_types::Wipe;

pub(super) fn check(w: &mut World, job: Wipe, now: u64) -> (Wipe, Option<End>) {
    w.truth.writes = 0;
    let id = w.ident();
    let ready = !not_ready(id) && id == w.dev.caps.ident;
    let skip = w.knob == Knob::NoMarker;
    let clear = |w: &mut World, b: u16| w.command() & b == 0;
    let confirmed = ready && (skip || job.marker.map(marker_bit).is_some_and(|b| clear(w, b)));
    let fall = job.method.and_then(|m| fallback(m, w.dev.caps.offers));
    match checked(job.phase, now, ready, confirmed, fall) {
        Next::Stay => (job, None),
        Next::Retry(m) => {
            w.seen[FALLBACK] += 1;
            (write(w, job, m, now), None)
        }
        Next::Ended(End::Clean) => {
            put_back(w, &job);
            (job, Some(End::Clean))
        }
        Next::Ended(end) => (job, Some(end)),
    }
}

/// restore.rs's BARs read back (they never move in the model), CLS, LT and
/// IntLine, Device Control without a reset asked or no-snoop, DevCtl2, and
/// the decoders as saved, Bus Master and INTx Disable left 0.
fn put_back(w: &mut World, job: &Wipe) {
    let Some(s) = job.saved else { return };
    (0..4).for_each(|_| w.touch(true));
    if let Some(d) = s.devctl {
        w.set_devctl(true, devctl_after(d));
        w.touch(true);
    }
    let keep = |v: u16| (v & !(IO | MEM | BME | INTX_DISABLE)) | (s.command & (IO | MEM));
    w.update_command(true, keep);
}

pub(super) fn unmark(w: &mut World, job: &Wipe) {
    let (Some(s), Some(m)) = (job.saved, job.marker) else { return };
    let bit = marker_bit(m);
    w.update_command(true, |v| (v & !bit) | (s.command & bit));
}
