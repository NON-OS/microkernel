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
use crate::app::App;
use crate::discover::{from_router, from_wm};

use super::super::control::{handle_control, ControlOutcome};
use super::super::dispatch::parse_delivery;
use super::super::drag::DragState;
use super::super::held::{next_message, Held};
use super::input::input;
use super::keys::keys_for;
use super::result::{DrainResult, Frame};

/// Take every message waiting, in order, and say what the frame loop must do.
/// A close, minimise or maximise ends the drain there; the rest gather.
#[allow(clippy::too_many_arguments)]
pub(in crate::runner) fn drain<A: App>(
    app: &mut A,
    drag_state: &mut DragState,
    held: &mut Option<Held>,
    rx: &mut [u8],
    frame: Frame,
    wm_port: u32,
    window_id: u32,
    request_id: &mut u32,
) -> DrainResult {
    let mut out = DrainResult::default();
    loop {
        let mut sender = 0u32;
        let n = next_message(held, rx, &mut sender);
        if n <= 0 {
            return out;
        }
        let msg = &rx[..n as usize];
        // The keyboard came or went: only the WM may say so, of this window.
        if let Some(has) = keys_for(msg, window_id).filter(|_| from_wm(sender)) {
            (out.focused, out.repaint) = (Some(has), true);
            continue;
        }
        match handle_control(msg, sender, wm_port, window_id, request_id) {
            ControlOutcome::FocusSelf => {
                out.restore = true;
                continue;
            }
            ControlOutcome::Handled => continue,
            ControlOutcome::NotControl => {}
        }
        // Routed input only: a frame any other process sent is not the user,
        // and goes to the app's peer_frame, which drops it unless it asked.
        if !from_router(sender) {
            out.repaint |= app.peer_frame(sender, msg);
            continue;
        }
        let Some(event) = parse_delivery(msg) else { continue };
        if input(app, drag_state, &mut out, event, frame, (wm_port, window_id, request_id)) {
            return out;
        }
    }
}
