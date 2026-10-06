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

//! Take a command the shell holds for this window (a Launchpad tool tile) and
//! put it in a tab: run as if it had been typed and Enter pressed, through
//! the same `on_enter` a key reaches, so it is echoed, kept in history and its
//! output lands in that tab; or left on the prompt with the cursor at its end
//! for the person to add arguments.

use nonos_libc::mk_time_millis;

use super::tabs::MAX_TABS;
use super::types::Terminal;
use crate::event::on_enter;
use crate::term::handed::{self, cadence, Active, Handed, Pick};
use crate::term::util::copy_into;

const BUSY: &[u8] = b": every tab is busy, close one and click the tile again";

impl Terminal {
    /// True when a tab changed.
    pub(super) fn take_handed(&mut self) -> bool {
        let now = mk_time_millis();
        if now < self.handed_due_ms {
            return false;
        }
        self.handed_due_ms = cadence::next_ask(now, self.started_ms);
        let Some(body) = handed::ask() else { return false };
        let Some(command) = handed::parse(&body) else { return false };
        let s = self.cur_ref();
        let active = Active {
            fresh: s.fresh,
            idle: s.line.as_bytes().is_empty() && !s.fg_running && s.search.is_none(),
        };
        match handed::pick(active, self.tabs.len(), MAX_TABS) {
            Pick::Current => {}
            Pick::NewTab => self.open_tab(),
            Pick::Nowhere => {
                let (Handed::Run(line) | Handed::Type(line)) = command;
                self.say_busy(line);
                return true;
            }
        }
        let state = self.cur();
        match command {
            Handed::Run(line) => {
                state.line.replace(line);
                // A tool name never asks the window to close; `exit` is the
                // only command that does, and the shell never hands it.
                let _ = on_enter(state);
                self.drain_chrome_req();
            }
            Handed::Type(line) => {
                state.line.replace(line);
                state.history.reset_cursor();
                state.scrollback.jump_bottom();
            }
        }
        true
    }

    fn say_busy(&mut self, line: &[u8]) {
        let mut msg = [0u8; 128];
        let head = line.len().min(msg.len() - BUSY.len());
        let mut n = copy_into(&mut msg, &line[..head]);
        n += copy_into(&mut msg[n..], BUSY);
        let state = self.cur();
        state.scrollback.push_line(&msg[..n]);
        state.scrollback.jump_bottom();
    }
}
