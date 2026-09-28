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

//! Finishing the Open prompt: the typed path opens in its own tab.

use super::app::Editor;
use nonos_app_skeleton::EventOutcome;

impl Editor {
    /// Finish the Open prompt: the typed path opens in its own tab, so the
    /// document the prompt was started from keeps its text and its name.
    pub(super) fn finish_open_prompt(&mut self) -> EventOutcome {
        let d = self.doc();
        let typed = d.prompt_path[..d.prompt_len].to_vec();
        d.prompt = None;
        d.prompt_len = 0;
        if typed.is_empty() {
            d.status = b"no path given";
            return EventOutcome::Repaint;
        }
        match core::str::from_utf8(&typed) {
            Ok(path) => {
                self.open_path(path);
            }
            Err(_) => self.doc().status = b"open refused: path is not valid UTF-8",
        }
        EventOutcome::Repaint
    }
}
