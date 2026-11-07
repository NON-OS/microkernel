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

//! Whether a run is the store's window run of a program made for a
//! terminal: it was asked for a window, its box grants TTY and not WAYLAND,
//! and the Terminal did not start it. Such a run hands its program to the
//! Terminal (terminal_hand.rs), so it takes no network role of its own
//! first: the Terminal's run takes one when its program needs it. Pure, so
//! the host proofs hold it.

use nonos_linux_policy::mask::{grants, TTY, WAYLAND};

pub fn for_terminal(window: bool, mask: u64, by_terminal: bool) -> bool {
    window && grants(mask, TTY) && !grants(mask, WAYLAND) && !by_terminal
}
