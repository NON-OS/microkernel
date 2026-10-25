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

//! The input menu's test model: the router's token, displays' offers, and
//! the rows the test expects them to list.

use crate::state::input_offers::Offer;
use crate::state::input_source_rule::{Routed, Router};

pub fn by() -> Routed {
    Router::Pinned(7).frame(7).expect("the router's own frame")
}

pub fn offer(s: u64, i: u32) -> Offer {
    let n = (s >> (i * 4)) as u8 % 5;
    Offer {
        identity: 0x8000_0000 | i,
        now: (s >> 30) as u16 % 4,
        inputs: [1, 2, 3, 4, 5, 6, 7, 8],
        n,
    }
}

/// The test's own menu: whether it is open, the keyboard's row, and each
/// row's panel, input and whether the display shows it now.
#[derive(Default)]
pub struct Model {
    pub open: bool,
    pub at: usize,
    pub rows: Vec<(u32, u8, bool)>,
}

pub fn rows(offers: &[Offer]) -> Vec<(u32, u8, bool)> {
    let listing = offers.iter().filter(|o| o.n > 0).take(3);
    listing
        .flat_map(|o| {
            o.inputs[..o.n as usize].iter().map(move |&v| (o.identity, v, v as u16 == o.now))
        })
        .collect()
}

/// The seeded generator's next state.
pub fn next(s: &mut u64) {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
}
