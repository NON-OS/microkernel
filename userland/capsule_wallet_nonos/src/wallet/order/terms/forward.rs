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

/*
 * The wallet asks to come forward as its consent opens: its own
 * window_raise and window_focus (guard/focus.rs). Under T1b the window
 * manager takes a client's own raise or focus only within 1 s of input
 * the router gave it, else answers E_PERM and the dock marks the client
 * for attention. A consent opened by the store's POST is not that, so
 * both are refused by design: bringing the wallet forward is the
 * person's act. A refusal, or any answer, is no failure and says
 * nothing: the consent shows drawn and unarmed until OP_QUERY_FOCUS says
 * the wallet holds focus, and its ARM_MS starts from that.
 */

use super::arming::Guard;

/// The consent's guard once the wallet asked to come forward, `raised`
/// and `focused` being what the window manager answered: the same, and
/// no failure, for a refusal as for a yes.
pub fn forward(raised: Result<(), &'static str>, focused: Result<(), &'static str>) -> Guard {
    let (Ok(()) | Err(_), Ok(()) | Err(_)) = (raised, focused);
    Guard::default()
}
