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
//! The two calls a loss proof judges bootfb by: a present, which
//! draws until the scanout is lost and answers ENXIO after; and STATE,
//! LOST from then on; and the fence where a takeover mid-copy lands.

use std::cell::Cell;

use super::serve::{ask, words, HELD};
use crate::display::BootFb;
use crate::pool::Pool;
use nonos_gfx_serve::Server;
use nonos_gfx_wire::body::state::LOST;
use nonos_gfx_wire::errno::ENXIO;
use nonos_gfx_wire::{ops, State, Surface};

type Srv = Server<BootFb, Pool>;

pub fn present(s: &mut Srv, sf: Surface, px: u32, seen: &Option<Vec<u32>>, tag: &str) {
    let base = s.slots[0].current.unwrap().region.base;
    s.platform.memory.get_mut(&base).unwrap().chunks_exact_mut(4).for_each(|c| c.copy_from_slice(&px.to_le_bytes()));
    let st = ask(s, ops::FLUSH, &words(&[sf.serial, 1, 0, 0, 16, 8])).0;
    assert_eq!(st == ENXIO, seen.is_some(), "{tag}: FLUSH {st}");
}

pub fn look(s: &mut Srv, seen: &Option<Vec<u32>>, tag: &str) {
    let st = State::decode(&ask(s, ops::STATE, &[]).1).unwrap();
    assert_eq!(st.flags & LOST != 0, seen.is_some(), "{tag}: STATE {st:?}");
}

thread_local! {
    pub static MID: Cell<bool> = const { Cell::new(false) };
}

/// The fence a present ends with: where a takeover mid-copy lands.
pub fn fence() {
    if MID.with(|m| m.replace(false)) {
        HELD.with(|h| h.set(false));
    }
}
