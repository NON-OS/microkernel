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

//! What the driver serves. The first card identified in the slot is the
//! disk. Two cards of one size cannot be told apart by size, so a card is
//! told from another by its CID (sd/cid.rs): once a card has been served,
//! no other is served in its place this run, and a card whose CID cannot
//! be read where the served one was ends service rather than being taken
//! for it. The kernel chose its disk once; a store write must never land
//! on a card the person swapped in.

use crate::card::Card;
use crate::protocol::{E_IO, E_NODEV, E_NOMEDIUM};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Medium {
    /// No card served yet, and none in the slot.
    Empty,
    /// No card served yet; the one in the slot did not come up, and is
    /// left powered down until it is taken out.
    Refused,
    Ready(Card),
    /// The served card failed a request, or the slot saw a change: it is
    /// identified again before it serves another request.
    Check(Card),
    /// The served card is out of the slot; it alone is served again.
    Away(Card),
    /// Another card, or one that could not be identified, sat where the
    /// served card was. Nothing is served again this run.
    Changed,
}

impl Medium {
    /// After the slot latched a change: the served card is in doubt, and a
    /// card that would not come up may be another one now.
    pub const fn shaken(self) -> Medium {
        match self {
            Medium::Ready(card) => Medium::Check(card),
            Medium::Refused => Medium::Empty,
            other => other,
        }
    }

    /// The status a request gets when there is no card to serve it.
    pub const fn refusal(&self) -> i32 {
        match self {
            Medium::Empty | Medium::Refused | Medium::Away(_) => E_NOMEDIUM,
            // Left unchecked only when the reader no longer answers.
            Medium::Ready(_) | Medium::Check(_) => E_IO,
            Medium::Changed => E_NODEV,
        }
    }
}
