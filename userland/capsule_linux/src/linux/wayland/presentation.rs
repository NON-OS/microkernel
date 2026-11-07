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

//! wp_presentation: when a client's frame reached the screen. A feedback
//! asked on a surface is for its next commit: presented once that commit's
//! pixels are on the window, discarded when they never will be (the surface
//! is not on the shown window, or goes). Presented is told when the
//! compositor shows the first frame after the pixels were handed to it, at
//! the time, refresh and sequence it reports (presentation_wait.rs), on the
//! guest's own CLOCK_MONOTONIC (call/clock.rs); at the hand-over itself,
//! with no refresh or sequence, when it reports none. No flag is set:
//! nothing claims vsync, a hardware clock or zero copy. The feedback is
//! gone with its event, and its id goes back to the client.

use alloc::vec::Vec;

use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::out::Event;

const DESTROY: u16 = 0;
const FEEDBACK: u16 = 1;
const CLOCK_ID: u16 = 0;
const SYNC_OUTPUT: u16 = 0;
const PRESENTED: u16 = 1;
const DISCARDED: u16 = 2;
const CLOCK_MONOTONIC: u32 = 1;

/// Bound: the clock its timestamps are on.
pub fn bound(guest: &mut Guest, id: u32) {
    Event::new(id, CLOCK_ID).u32(CLOCK_MONOTONIC).send(&mut guest.display.to_client);
}

/// wp_presentation's requests. False means nothing served it.
pub fn route(guest: &mut Guest, object: Option<Object>, id: u32, op: u16, args: &mut Args) -> bool {
    match (object, op) {
        (Some(Object::Presentation), DESTROY) => super::forget::forget(guest, object, id),
        (Some(Object::Presentation), FEEDBACK) => {
            let (Some(surface), Some(feedback)) = (args.u32(), args.u32()) else { return true };
            if !guest.objects.put(feedback, Object::PresentationFeedback) {
                return true;
            }
            match guest.scene.surfaces.iter_mut().find(|s| s.id == surface) {
                Some(s) => s.feedback.push(feedback),
                None => discard(guest, feedback),
            }
        }
        _ => return false,
    }
    true
}

/// `surface` committed: its feedbacks told whether that commit is shown.
pub fn committed(guest: &mut Guest, surface: u32) {
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface) else { return };
    if s.feedback.is_empty() {
        return;
    }
    let feedback = core::mem::take(&mut s.feedback);
    match on_window(guest, surface) {
        true => {
            feedback.into_iter().for_each(|f| super::presentation_wait::wait(guest, f, surface))
        }
        false => feedback.into_iter().for_each(|f| discard(guest, f)),
    }
}

/// `surface` going: what it was asked is never shown.
pub fn gone(guest: &mut Guest, surface: u32) {
    super::presentation_wait::gone(guest, surface);
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface) else { return };
    let feedback: Vec<u32> = core::mem::take(&mut s.feedback);
    feedback.into_iter().for_each(|f| discard(guest, f));
}

/// Whether `surface` is the shown window's, or laid on it (a subsurface,
/// a popup), however deep.
fn on_window(guest: &Guest, surface: u32) -> bool {
    let shown = guest.scene.win.shown.map(|w| w.surface);
    let mut at = surface;
    for _ in 0..super::subsurface::order::MAX_DEPTH {
        if Some(at) == shown {
            return true;
        }
        match guest.scene.subs.of(at) {
            Some(sub) => at = sub.parent,
            None => return false,
        }
    }
    false
}

/// Presented at the moment the pixels were handed over, with no refresh
/// and no sequence: when the compositor cannot say when it showed them.
pub(super) fn presented_now(guest: &mut Guest, feedback: u32) {
    let ms = now_ms(u64::from(CLOCK_MONOTONIC)).unwrap_or(0);
    presented(guest, feedback, ms.saturating_mul(1_000_000), (0, 0));
}

/// Presented at `ns` on the guest's clock, with the output's `refresh`
/// period in nanoseconds and the compositor's frame `seq`, 0 when unknown.
/// No flag: nothing here claims vsync, a hardware clock or zero copy.
pub(super) fn presented(guest: &mut Guest, feedback: u32, ns: u64, (refresh, seq): (u32, u64)) {
    let outputs: Vec<u32> = guest.objects.all(Object::Output).collect();
    let to = &mut guest.display.to_client;
    outputs.iter().for_each(|&o| Event::new(feedback, SYNC_OUTPUT).u32(o).send(to));
    let (secs, nanos) = (ns / 1_000_000_000, ns % 1_000_000_000);
    Event::new(feedback, PRESENTED)
        .u32((secs >> 32) as u32)
        .u32(secs as u32)
        .u32(nanos as u32)
        .u32(refresh)
        .u32((seq >> 32) as u32)
        .u32(seq as u32)
        .u32(0)
        .send(to);
    done(guest, feedback);
}

pub(super) fn discard(guest: &mut Guest, feedback: u32) {
    Event::new(feedback, DISCARDED).send(&mut guest.display.to_client);
    done(guest, feedback);
}

fn done(guest: &mut Guest, feedback: u32) {
    guest.objects.drop_id(feedback);
    super::handlers::delete_id(guest, feedback);
}
