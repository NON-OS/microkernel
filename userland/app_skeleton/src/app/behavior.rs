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

use super::{AppManifest, EventOutcome};
use crate::input::InputEvent;
use crate::paint::PaintBuffer;
use nonos_toolkit::decorations::Rect;

pub trait App {
    fn manifest(&self) -> AppManifest;
    fn on_event(&mut self, event: InputEvent) -> EventOutcome;
    fn paint(&mut self, fb: &mut PaintBuffer);

    fn on_tick(&mut self) -> bool {
        false
    }

    fn tick_interval_ms(&self) -> i64 {
        1000
    }

    /// Whether the app has pending asynchronous work (a request in flight, a
    /// load) and must keep ticking promptly: the runner then yields between
    /// frames instead of sleeping to the next vblank, so the work advances where
    /// the scheduler's wake is unreliable; under `run` each wait is capped at a
    /// millisecond. Defaults to idle.
    fn busy(&self) -> bool {
        false
    }

    /// The part of the content area that changed since the last paint, in
    /// content coordinates, or None when the whole window must be redrawn.
    /// Called right before `paint`; an app that returns a rect promises its
    /// next `paint` redraws every pixel inside it and touches nothing else,
    /// so the runner skips the frame and commits only that rect. The
    /// default keeps the whole-window repaint every app had before.
    fn take_damage(&mut self) -> Option<Rect> {
        None
    }

    /// Width of an app-owned widget right aligned in the titlebar; zero, the
    /// default, keeps the whole bar. A width moves the title clear of it, hands
    /// `paint_accessory` a sub-buffer over it, and sends pointer events inside
    /// it to `on_accessory_event` rather than a drag; keys still go through
    /// `on_event`, so an accessory that takes text tracks its own focus.
    fn titlebar_accessory_w(&self) -> u32 {
        0
    }

    fn paint_accessory(&mut self, _fb: &mut PaintBuffer) {}

    fn on_accessory_event(&mut self, _event: InputEvent) -> EventOutcome {
        EventOutcome::Idle
    }

    /// Whether the app wants its window full screen: the green button's
    /// full screen, the whole display below the menubar with the dock hidden.
    /// Asked every frame. True from the start opens the window full screen;
    /// turning true later (Video while it plays) takes it full screen, and
    /// turning false gives back the rect it had, unless the person chose full
    /// screen themselves with green. Defaults to never asking.
    fn wants_full_screen(&self) -> bool {
        false
    }

    /// A rect the window should stand at, in thousandths of the work area
    /// (runner/rect_ask.rs); None, the default, leaves the window alone.
    fn wants_rect(&self) -> Option<crate::runner::work_rect::WorkRect> {
        None
    }

    /// Asked when the window's close button is pressed. An app holding work
    /// the user would lose returns false, says so in its own window, and the
    /// window stays open; pressing close again is the user's answer. Defaults
    /// to closing.
    fn close_requested(&mut self) -> bool {
        true
    }

    /// A frame from a process other than the input router: an answer the app
    /// asked a service for without waiting. True repaints. Defaults to
    /// dropping it unread, as every frame not from the router always was.
    fn peer_frame(&mut self, _sender: u32, _frame: &[u8]) -> bool {
        false
    }
}
