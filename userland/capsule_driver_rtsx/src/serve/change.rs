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

//! The medium moved on by what is in the slot. A first card is brought up
//! and served. The served card taken out powers the slot down; a card put
//! back, or the served card in doubt, is powered down, brought up afresh
//! and served only if its CID and size are the ones served. Any other
//! card, or one that does not come up far enough to be told, ends service.

use super::medium::Medium;
use crate::card::{bring_up, power_off, Card};
use crate::error::RtsxError;
use crate::log::failed;
use crate::report::{said, served};
use crate::setup::Driver;

pub fn next(drv: &mut Driver, medium: Medium, here: bool) -> Medium {
    match (medium, here) {
        (Medium::Empty, true) => first(drv),
        (Medium::Refused, false) => Medium::Empty,
        (Medium::Ready(card) | Medium::Check(card), false) => {
            let _ = power_off(drv);
            said(b"the served card was taken out; slot powered down");
            Medium::Away(card)
        }
        (Medium::Check(card) | Medium::Away(card), true) => again(drv, card),
        (medium, _) => medium,
    }
}

fn first(drv: &mut Driver) -> Medium {
    match bring_up(drv) {
        Ok(card) => {
            served(drv, &card);
            Medium::Ready(card)
        }
        Err(e) => {
            let _ = power_off(drv);
            failed(b"card left powered down until it is taken out", e);
            Medium::Refused
        }
    }
}

fn again(drv: &mut Driver, served: Card) -> Medium {
    let _ = power_off(drv);
    let next = match bring_up(drv) {
        Ok(card) if card.same(&served) => {
            said(b"the served card is in the slot again; served");
            return Medium::Ready(card);
        }
        Err(RtsxError::Removed) => Medium::Away(served),
        _ => {
            said(b"another card where the served one was, or one not told apart: service ended");
            Medium::Changed
        }
    };
    let _ = power_off(drv);
    next
}
