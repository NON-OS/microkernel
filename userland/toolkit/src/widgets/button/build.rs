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

//! The button and the builders that set it up.

use super::Kind;
use crate::design::size::ICON;
use crate::icons::IconId;
use crate::widgets::State;

/// A push button with a one-line label, and an icon when one is set.
#[derive(Clone, Copy, Debug)]
pub struct Button<'a> {
    pub(super) label: &'a str,
    pub(super) kind: Kind,
    pub(super) state: State,
    pub(super) icon: Option<IconId>,
    pub(super) side: u32,
}

impl<'a> Button<'a> {
    /// A `Secondary` button showing `label`.
    pub fn new(label: &'a str) -> Self {
        let (kind, state) = (Kind::default(), State::default());
        Button { label, kind, state, icon: None, side: ICON }
    }

    /// The kind of button, which decides its colours.
    pub fn kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }

    /// An icon in the label's ink: centred alone when the label is empty,
    /// else leading the label, the pair centred as one group.
    pub fn icon(mut self, id: IconId) -> Self {
        self.icon = Some(id);
        self
    }

    /// The icon's side in logical pixels, `ICON` unless set, as on an
    /// `IconButton`.
    pub fn icon_size(mut self, side: u32) -> Self {
        self.side = side;
        self
    }

    /// The state the app reports. `selected` arms a `Danger` button, turns a
    /// `Plain` or `Quiet` toggle on, and is ignored by the other kinds.
    pub fn state(mut self, s: State) -> Self {
        self.state = s;
        self
    }
}
