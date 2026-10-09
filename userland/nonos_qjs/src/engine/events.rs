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

use alloc::string::String;

use super::ffi::{
    njs_dispatch_event, njs_event_default_prevented, njs_flush_timers, njs_take_history_step,
    njs_take_navigation, njs_viewport_changed,
};
use super::lifecycle::Engine;

impl Engine {
    /// Dispatch a UI event of `ty` to the listeners registered on `node`,
    /// returning how many fired. Used when a real pointer or key event lands on
    /// a laid-out node; the engine must be the one that ran the page scripts.
    pub fn dispatch_event(&self, node: i32, ty: &str) -> i32 {
        let mut buf = [0u8; 32];
        let n = ty.len().min(31);
        buf[..n].copy_from_slice(&ty.as_bytes()[..n]);
        unsafe { njs_dispatch_event(self.ctx, node, buf.as_ptr()) }
    }

    /// Whether a listener of the last dispatched event called preventDefault.
    /// The browser then skips the event's own action (following the link,
    /// focusing the field, submitting the form), and only then.
    pub fn default_prevented(&self) -> bool {
        unsafe { njs_event_default_prevented() != 0 }
    }

    /// Run the page timers due at `now_ms`, and report how many ran.
    ///
    /// The caller passes real elapsed milliseconds rather than letting the
    /// queue decide what is due. A queue that decides for itself has to move
    /// its own clock to whatever comes next, which makes a repeating timer
    /// due again the instant it is requeued: one `setInterval` then runs to
    /// the iteration cap on every tick and the page never stops working long
    /// enough to draw.
    pub fn flush_timers(&self, now_ms: u64) -> i32 {
        unsafe { njs_flush_timers(self.ctx, now_ms as f64) }
    }

    /// The address a script asked to navigate to since this was last called.
    ///
    /// Navigating from inside the run would tear down the tree the script is
    /// still executing against, so the request is parked and collected once
    /// the run is over. Reading clears it, so one request is acted on once
    /// rather than on every poll after it.
    pub fn take_navigation(&self) -> Option<String> {
        unsafe {
            let p = njs_take_navigation();
            if p.is_null() {
                return None;
            }
            let mut n = 0;
            while *p.add(n) != 0 {
                n += 1;
            }
            Some(String::from_utf8_lossy(core::slice::from_raw_parts(p, n)).into_owned())
        }
    }

    /// The step through the reader's history a script asked for since this
    /// was last called (`history.back()` is -1, `forward()` 1, `go(n)` n),
    /// parked like a navigation and cleared by reading.
    pub fn take_history_step(&self) -> Option<i32> {
        match unsafe { njs_take_history_step() } {
            0 => None,
            n => Some(n),
        }
    }

    /// The window was resized and the document's viewport is the new size:
    /// each media query list the page listens to whose answer changed hears
    /// `change`, then window hears `resize`. Runs on the time budget.
    pub fn viewport_changed(&self) {
        unsafe { njs_viewport_changed(self.ctx) }
    }
}
