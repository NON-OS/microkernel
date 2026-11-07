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

//! wl_subsurface's requests. Where a subsurface sits and where it stands
//! among its siblings are taken at its parent's next commit (commit.rs);
//! set_desync shows at once a commit that was waiting for the parent.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::req;
use super::compose::repaint;

/// The place_above and place_below requests kept between two commits.
pub const MOST_RESTACKS: usize = 256;

/// A request on the wl_subsurface `id`; false when nothing served it.
pub fn route(guest: &mut Guest, id: u32, op: u16, args: &mut Args<'_>) -> bool {
    let subs = &mut guest.scene.subs;
    // One whose parent is gone is inert: its requests change nothing.
    let Some(sub) = subs.list.iter_mut().find(|s| s.id == id) else {
        return true;
    };
    match op {
        req::SUBSURFACE_SET_POSITION => {
            let (Some(x), Some(y)) = (args.u32(), args.u32()) else { return true };
            sub.next_at = Some((x as i32, y as i32));
        }
        req::SUBSURFACE_PLACE_ABOVE | req::SUBSURFACE_PLACE_BELOW => {
            let Some(reference) = args.u32() else { return true };
            let (surface, parent) = (sub.surface, sub.parent);
            // Only the parent or a sibling: anything else is the client's error.
            let sibling = subs.of(reference).is_some_and(|r| r.parent == parent);
            // Applied in order at the parent's commit, so they are kept, up to
            // a bound: more between two commits is no client drawing a window.
            let room = subs.restack.len() < MOST_RESTACKS;
            if room && reference != surface && (reference == parent || sibling) {
                let above = op == req::SUBSURFACE_PLACE_ABOVE;
                subs.restack.push((surface, reference, above));
            }
        }
        req::SUBSURFACE_SET_SYNC => sub.sync = true,
        req::SUBSURFACE_SET_DESYNC => {
            sub.sync = false;
            if let Some(waiting) = sub.waiting.take() {
                // Nothing for nothing changes no pixel.
                let changed = sub.shown.is_some() || waiting.is_some();
                sub.shown = waiting;
                if changed {
                    repaint(guest);
                }
            }
        }
        _ => return false,
    }
    true
}
