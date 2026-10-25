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

//! Where the takeover model is searched from (L79): which paths take part,
//! whether bootfb claims a second time, and how many draws the kernel tries.

use super::world::{Page, END};
use super::world_state::{Taker, World};

/// What B's sibling thread does once B's first map has returned: nothing,
/// unmap that grant through the broker, or have the grant's address show
/// something else by a path outside the grant table. None exists today
/// (munmap refuses the device window); the model lets it happen, so the
/// repoint's own check is what keeps such a page.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Sib {
    Idle,
    Unmap,
    Stray,
}

/// G2 takes part; G1's holder exits while its claim is being made; B claims
/// again after its first round; its sibling's part; bootfb exits; draws.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) struct Cfg {
    pub g2: bool,
    pub doomed: bool,
    pub again: bool,
    pub sib: Sib,
    pub exit: bool,
    pub draws: u8,
}

pub(super) fn start(cfg: Cfg) -> World {
    let on = |x: bool| if x { 0 } else { END };
    let sib = on(cfg.sib != Sib::Idle);
    World {
        fb: None,
        gpu: [None; 2],
        grant: [false; 2],
        page: [Page::Nothing; 2],
        held: false,
        gone: false,
        takers: 0,
        draws: 0,
        pc: [0, on(cfg.g2), 0, sib, on(cfg.exit), on(cfg.draws > 0), on(cfg.doomed)],
        g: [Taker::default(); 2],
        round: 0,
        gb: (0, 0),
        mapped: false,
        inside: false,
        exit: None,
        dead: false,
        freed: false,
        ever_in: false,
        cfg,
        events: 0,
        bad: 0,
    }
}
