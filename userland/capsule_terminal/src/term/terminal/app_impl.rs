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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::types::Terminal;

impl App for Terminal {
    fn manifest(&self) -> AppManifest {
        self.manifest_inner()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        let before = self.frame_key();
        let key = event.kind == nonos_app_skeleton::InputKind::KeyDown;
        // The clock is read for a key-down alone: pointer motion comes far
        // more often and is never timed (key_timing.rs).
        let now = if key { nonos_libc::mk_uptime_ms().max(1) } else { 0 };
        let was = self.key_at;
        let outcome = self.on_event_inner(event);
        let repaint = outcome == EventOutcome::Repaint;
        self.key_at = super::key_timing::timed_from(was, now, key, repaint);
        self.line_only = key && outcome == EventOutcome::Repaint && self.frame_key() == before;
        self.uptime_only = false;
        self.whole_owed |= outcome == EventOutcome::Repaint && !self.line_only;
        if outcome == EventOutcome::Close {
            self.hang_up_all();
        }
        outcome
    }

    fn wants_rect(&self) -> Option<nonos_app_skeleton::WorkRect> {
        crate::term::drop::rect(self.dropped)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.paint_inner(fb)
    }

    fn on_tick(&mut self) -> bool {
        let repaint = self.on_tick_inner();
        self.line_only &= !repaint;
        repaint
    }

    fn take_damage(&mut self) -> Option<crate::layout::Rect> {
        self.take_damage_inner()
    }

    fn paint_whole_next(&mut self) {
        // The input row alone would land on a buffer out of date elsewhere.
        self.partial = false;
    }

    fn painted_damage_alone(&self) -> bool {
        self.row_painted
    }

    fn tick_interval_ms(&self) -> i64 {
        30
    }

    fn titlebar_accessory_w(&self) -> u32 {
        self.accessory_w()
    }

    fn paint_accessory(&mut self, fb: &mut PaintBuffer) {
        self.paint_accessory_inner(fb)
    }

    fn on_accessory_event(&mut self, event: InputEvent) -> EventOutcome {
        self.accessory_event(event)
    }

    fn close_requested(&mut self) -> bool {
        self.hang_up_all();
        true
    }
}
