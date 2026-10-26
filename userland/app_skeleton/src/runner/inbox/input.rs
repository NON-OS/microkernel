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
//! One routed input event: the frame's buttons, the title bar accessory, a
//! drag of the frame, and then the app.

use nonos_toolkit::decorations::accessory_rect_at;

use crate::app::{App, EventOutcome};
use crate::input::InputEvent;

use super::super::drag::{self, DragState, PointerAction};
use super::super::press_part::Route;
use super::super::{click_focus, decorations};
use super::buttons::{buttons, stop};
use super::result::{DrainResult, Frame};

/// Handle `event` into `out`; true when it ends the drain (close,
/// minimise, maximise). `wm` is the WM's port, the window and the request id.
pub(super) fn input<A: App>(
    app: &mut A,
    drag_state: &mut DragState,
    out: &mut DrainResult,
    event: InputEvent,
    f: Frame,
    wm: (u32, u32, &mut u32),
) -> bool {
    let event = decorations::normalize(event);
    click_focus::handle(event, wm.0, wm.1, wm.2);
    if let Some(ends) = buttons(app, out, f, event) {
        return ends;
    }
    let q = super::super::chrome::quarters();
    let accessory =
        accessory_rect_at(f.width, f.height, f.maximized, app.titlebar_accessory_w(), q);
    let event = match drag_state.press.route(event, accessory) {
        Route::Accessory(local) => {
            out.repaint |= app.on_accessory_event(local) == EventOutcome::Repaint;
            return false;
        }
        Route::Window(event) => event,
    };
    match drag::handle(drag_state, (f.width, f.height), (f.win_x, f.win_y), f.maximized, &event) {
        PointerAction::MoveTo(mx, my) => out.move_to = Some((mx, my)),
        PointerAction::ResizeTo(rw, rh) => out.resize_to = Some((rw, rh)),
        PointerAction::Place(p) => out.place = Some(p),
        PointerAction::HoverChanged => out.repaint = true,
        PointerAction::Held => {}
        PointerAction::None => {
            let event = decorations::to_content(f.width, f.height, f.maximized, event);
            match app.on_event(event) {
                EventOutcome::Repaint => out.repaint = true,
                EventOutcome::Close => return stop(&mut out.close),
                _ => {}
            }
        }
    }
    false
}
