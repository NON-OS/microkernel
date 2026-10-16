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

//! What the input menu does with what the displays offer and with the
//! router's keys and presses (`input_menu.rs` holds the rule's types).

use super::dialog_keys::{KEY_ENTER, KEY_ESC};
use super::input_menu::{InputMenu, Switch, KEY_DOWN, KEY_UP};
use super::input_offers::{Offer, Row, MAX_DISPLAYS, MAX_INPUTS};
use super::input_source_rule::Routed;

impl InputMenu {
    /// Open on the displays that list an input, the keyboard on the first
    /// row. Nothing opens when none does.
    pub fn open(&mut self, offers: &[Offer]) -> bool {
        *self = InputMenu::default();
        for o in offers.iter().filter(|o| o.n > 0).take(MAX_DISPLAYS) {
            (self.offers[self.shown], self.shown) = (*o, self.shown + 1);
        }
        self.open = self.shown > 0;
        self.open
    }

    pub fn showing(&self) -> bool {
        self.open
    }

    /// The row the keyboard is on.
    pub fn at(&self) -> usize {
        self.at
    }

    pub fn rows(&self) -> impl Iterator<Item = Row> + '_ {
        self.offers[..self.shown].iter().enumerate().flat_map(|(display, o)| {
            let listed = &o.inputs[..(o.n as usize).min(MAX_INPUTS)];
            listed.iter().map(move |&input| Row {
                display,
                input,
                showing: u16::from(input) == o.now,
            })
        })
    }

    /// Up and Down move the keyboard, Enter chooses its row, Esc closes.
    pub fn key(&mut self, by: Routed, code: u32) -> Option<Switch> {
        let last = self.rows().count().saturating_sub(1);
        match code {
            _ if !self.open => return None,
            KEY_ENTER => return self.choose(by, Some(self.at)),
            KEY_ESC => return self.choose(by, None),
            KEY_UP => self.at = self.at.saturating_sub(1),
            KEY_DOWN => self.at = (self.at + 1).min(last),
            _ => {}
        }
        None
    }
}
