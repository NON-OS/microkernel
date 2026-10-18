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

//! The display input menu. Switching a display to another input takes the
//! picture away from this machine, so no key, timer or message does it: a
//! person opens the menu from the system menu, the shell lists in the
//! trusted band only the inputs each display's own capabilities list, and
//! only a choice made with the router's input switches, once.

use super::input_offers::{Offer, MAX_DISPLAYS};
use super::input_source_rule::Routed;

pub const NONE_OFFERED: &[u8] = b"No display here offers a choice of input";
pub const SWITCHED: &[u8] = b"Input switched; switch back on the display";
pub const REFUSED: &[u8] = b"The display did not switch its input";
pub const KEY_UP: u32 = 0x1201;
pub const KEY_DOWN: u32 = 0x1202;

/// Permission to switch one display to one input. Made only by
/// `InputMenu::choose`, for router input on an open menu; neither Clone nor
/// Copy, so the sender consumes it and one choice switches at most once.
#[derive(Debug, PartialEq, Eq)]
pub struct Switch {
    identity: u32,
    input: u8,
}

impl Switch {
    pub fn identity(&self) -> u32 {
        self.identity
    }

    pub fn input(&self) -> u8 {
        self.input
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputMenu {
    pub(super) offers: [Offer; MAX_DISPLAYS],
    pub(super) shown: usize,
    pub(super) at: usize,
    pub(super) open: bool,
}

impl InputMenu {
    /// A choice closes the menu. A row the display already shows, or a
    /// press outside every row (None), switches nothing.
    pub fn choose(&mut self, _by: Routed, row: Option<usize>) -> Option<Switch> {
        let was = core::mem::replace(&mut self.open, false);
        let r = self.rows().nth(row?).filter(|r| was && !r.showing)?;
        Some(Switch { identity: self.offers[r.display].identity, input: r.input })
    }
}
