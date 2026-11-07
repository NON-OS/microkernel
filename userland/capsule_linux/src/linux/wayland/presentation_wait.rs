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

//! A presentation feedback held until the compositor shows the frame it was
//! asked for. The compositor counts the frames it shows (OP_FRAME_TIME's
//! seq): the frame count when the pixels were handed over is kept, and the
//! first count past it is the frame that shows them, at the time, refresh
//! and sequence the compositor reports. One it never shows within GIVE_UP_MS
//! (nothing on screen changed, the window was covered) is discarded.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::compositor::{frame_time, FrameTime};
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::call::{family_ms, family_ns};
use crate::linux::guest::Guest;

use super::commit::pace::period;
use super::presentation::{discard, presented, presented_now};

/// How long a feedback waits for its frame to be shown.
const GIVE_UP_MS: u64 = 500;

pub struct Waiting {
    feedback: u32,
    surface: u32,
    /// The compositor's frame count when the pixels were handed over.
    seq: u64,
    since: u64,
}

/// `feedback`, for a commit of `surface` just handed to the compositor.
pub fn wait(guest: &mut Guest, feedback: u32, surface: u32) {
    match shown_so_far() {
        Some(t) => {
            let w = Waiting { feedback, surface, seq: t.seq, since: family_ms() };
            guest.scene.presenting.push(w);
        }
        None => presented_now(guest, feedback),
    }
}

/// The feedbacks whose frame the compositor has shown since, told so; those
/// waited for too long discarded.
pub fn settle(guest: &mut Guest, now: u64) {
    if guest.scene.presenting.is_empty() {
        return;
    }
    let t = shown_so_far();
    let refresh = t.map_or(0, |t| period(t.refresh_ns) as u32);
    for w in core::mem::take(&mut guest.scene.presenting) {
        match t {
            Some(t) if t.seq > w.seq => {
                presented(guest, w.feedback, family_ns(t.at_ns), (refresh, t.seq))
            }
            Some(_) if now.saturating_sub(w.since) < GIVE_UP_MS => guest.scene.presenting.push(w),
            Some(_) => discard(guest, w.feedback),
            None => presented_now(guest, w.feedback),
        }
    }
}

/// How long until the compositor is asked again, while a feedback waits.
pub fn next_ms(guest: &Guest, now: u64) -> Option<u64> {
    let half = period(guest.scene.refresh_ns) / 2_000_000;
    let ages = guest.scene.presenting.iter().map(|w| now.saturating_sub(w.since));
    ages.map(|age| ask_in(age, half)).min()
}

/// The wait before asking again for a feedback `age` ms old. The compositor
/// composes on the turn after a present, not on a refresh, so a frame is
/// usually shown a millisecond or two after it was handed over: it is asked
/// after one, then after as long again as it has waited, and never less
/// often than each half refresh (`half` ms). A frame shown is told within
/// about its own age rather than up to half a refresh late, for at most a
/// few more asks than one each half refresh.
pub fn ask_in(age: u64, half: u64) -> u64 {
    age.clamp(1, half.max(1))
}

/// `surface` going: what it waited for is never shown.
pub fn gone(guest: &mut Guest, surface: u32) {
    let (gone, kept): (Vec<Waiting>, Vec<Waiting>) = core::mem::take(&mut guest.scene.presenting)
        .into_iter()
        .partition(|w| w.surface == surface);
    guest.scene.presenting = kept;
    gone.into_iter().for_each(|w| discard(guest, w.feedback));
}

fn shown_so_far() -> Option<FrameTime> {
    lookup_port(b"compositor").and_then(|port| frame_time(port, 1).ok())
}
