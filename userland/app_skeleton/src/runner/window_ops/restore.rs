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

use nonos_toolkit::decorations::DecorationHit;

use crate::app::App;
use crate::clients::{compositor, wm};
use crate::discover::Peers;

use super::super::boot::BootedApp;
use super::super::hide_steps::{hide, hover_after_hide, show, HideSteps, ShowSteps};
use super::super::repaint::repaint;
use super::super::request_id::next;

const APP_LAYER_Z: u32 = 2;

struct Calls<'a, A: App> {
    booted: &'a mut BootedApp<A>,
    peers: &'a Peers,
    request_id: &'a mut u32,
}

impl<A: App> HideSteps for Calls<'_, A> {
    fn wm_minimize(&mut self) -> bool {
        let id = self.booted.manifest.window_id;
        wm::window_minimize(self.peers.wm, next(self.request_id), id).is_ok()
    }

    fn scene_remove(&mut self) -> bool {
        compositor::scene_remove(self.peers.compositor, next(self.request_id), 0).is_ok()
    }
}

impl<A: App> ShowSteps for Calls<'_, A> {
    fn scene_submit(&mut self) -> bool {
        let (peers, id) = (self.peers, self.booted.manifest.window_id);
        let b = &self.booted.binding;
        let (w, h) = (b.width, b.height);
        // Kept on the display, as a move is: it may have shrunk while the
        // window was on the dock, and a layer past its edge is refused.
        let (x, y) = match compositor::display_info(peers.compositor, next(self.request_id)) {
            Ok(di) => (b.x.min(di.width.saturating_sub(w)), b.y.min(di.height.saturating_sub(h))),
            Err(_) => (b.x, b.y),
        };
        let handle = b.surface_handle();
        let rid = next(self.request_id);
        if compositor::scene_submit(peers.compositor, rid, handle, x, y, w, h, APP_LAYER_Z).is_err()
        {
            return false;
        }
        if (x, y) != (b.x, b.y) {
            let _ = wm::window_move(peers.wm, next(self.request_id), id, x, y);
            (self.booted.binding.x, self.booted.binding.y) = (x, y);
        }
        true
    }

    fn wm_raise(&mut self) {
        let id = self.booted.manifest.window_id;
        let _ = wm::window_raise(self.peers.wm, next(self.request_id), id);
    }

    fn wm_minimize(&mut self) -> bool {
        HideSteps::wm_minimize(self)
    }

    fn repaint(&mut self) {
        repaint(self.booted, self.peers, self.request_id);
    }
}

/// Hide the window to the dock, as hide_steps.rs orders it.
pub(in crate::runner) fn minimize<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) {
    let hidden = hide(&mut Calls { booted: &mut *booted, peers, request_id });
    booted.drag.hover = hover_after_hide(hidden, booted.drag.hover, DecorationHit::None);
    if hidden {
        booted.minimized = true;
    }
}

/// Put a minimized window back in the scene where it was and repaint it,
/// as hide_steps.rs orders it.
pub(in crate::runner) fn restore<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) {
    // Shown from the dock, so no pointer is over a frame control; a pointer
    // event the hidden window took after its minimize may have lit one.
    booted.drag.hover = DecorationHit::None;
    if show(&mut Calls { booted: &mut *booted, peers, request_id }) {
        booted.minimized = false;
    }
}
