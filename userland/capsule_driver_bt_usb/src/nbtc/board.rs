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

//! What NBTC reports and what it changes: the bonds, and what the link
//! last told of itself. The link writes it through the capsule's `Env` and
//! every request reads it between two turns of the link, so the two never
//! hold it at once.

use alloc::vec::Vec;

use super::ops::IDLE;
use super::part::{Identity, Level, Part};
use crate::link::Answer;
use nonos_bt_bond::Store;

pub struct Board<K> {
    pub bonds: Store<K>,
    pub controller: bool,
    pub state: u8,
    /// Keyboard, mouse, and whether keys post, once HID is ready.
    pub ready: Option<(bool, bool, bool)>,
    /// The identity of the bond the link runs on.
    pub current: Option<Identity>,
    pub scan: bool,
    /// When the pairing window closes; 0 when shut.
    pub pair_until: u64,
    pub part: Part,
    pub answer: Option<Answer>,
    /// The link's bond was forgotten: the link is to end.
    pub drop_link: bool,
    pub levels: Vec<Level>,
}

impl<K> Board<K> {
    /// Scanning starts on, so a bonded device reconnects on its own; the
    /// pairing window starts shut.
    pub fn new(bonds: Store<K>) -> Self {
        Self {
            bonds,
            controller: false,
            state: IDLE,
            ready: None,
            current: None,
            scan: true,
            pair_until: 0,
            part: Part::None,
            answer: None,
            drop_link: false,
            levels: Vec::new(),
        }
    }
}
