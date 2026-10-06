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

//! The document's readyState, moved on by the browser as the page loads.

use super::lifecycle::Engine;

impl Engine {
    /*
     * The state lives in the prelude (dom_prelude_3.inc), with the
     * listeners it fires. A page reading readyState gets "loading" while
     * its scripts run, "interactive" once they all have (with
     * DOMContentLoaded), and "complete" after that (with load). A step
     * asked for twice, or out of turn, does nothing.
     */
    /// Move readyState on: to "interactive" and DOMContentLoaded, or with
    /// `complete` to "complete" and load. True if it moved, so listeners
    /// may have changed the page.
    pub fn advance_ready_state(&self, complete: bool) -> bool {
        let code = if complete {
            "typeof __njs_ready==='function'?__njs_ready('complete'):0"
        } else {
            "typeof __njs_ready==='function'?__njs_ready('interactive'):0"
        };
        self.eval(code) == "1"
    }
}
