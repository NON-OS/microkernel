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


//! The ticket a Terminal's `linux` run is given to start again with Network.
//! Pure, so a host proof holds it as it ships (term_ticket.rs keeps one).

use alloc::string::String;
use alloc::vec::Vec;

/// What the kernel wrote when its own app.linux.term child asked: the
/// Terminal that child is parented to, and the argv the kernel set at its
/// spawn, which the child cannot change and never names here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ticket {
    pub terminal: u32,
    pub argv: Vec<String>,
}

/// Whether `caller` may start `argv` in the terminal's network role on the
/// ticket in `slot`: only the ticket's own Terminal, only for the argv
/// byte for byte. The ticket is used up on its first check, whatever the
/// check finds, so a wrong ask cannot be followed by a right one.
pub fn admit(slot: &mut Option<Ticket>, caller: u32, argv: &[String]) -> bool {
    slot.take().is_some_and(|t| t.terminal == caller && t.argv == argv)
}

/// `terminal` is exiting: its ticket, if it holds one, goes with it.
pub fn end_of(slot: &mut Option<Ticket>, terminal: u32) {
    if slot.as_ref().is_some_and(|t| t.terminal == terminal) {
        *slot = None;
    }
}
