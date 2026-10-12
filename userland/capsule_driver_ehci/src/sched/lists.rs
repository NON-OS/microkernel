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

//! The lists the controller walks, built before it runs. The async ring:
//! its head (H, never with qTDs), address 0's control QH, each slot's
//! control QH and bulk QHs, back to the head; every one empty. The
//! periodic list: every frame entry at the first interrupt QH, the eight
//! chained; each without a micro-frame mask, so none runs until armed.

use super::layout::*;
use super::qh::{chars, init, qh_link};
use crate::env::Mem;
use crate::facts::qh::{CAP_MULT_1, CH_H, LINK_T};

pub fn build<M: Mem>(m: &M) {
    for i in 0..RING {
        let mut ch = chars(0, 0, 64, i == ADDR0_RING);
        if i == ASYNC_HEAD {
            ch |= CH_H;
        }
        init(m, qh(i), qh_link(m, qh((i + 1) % RING)), ch, CAP_MULT_1);
    }
    for i in 0..INTR_MAX {
        let next = if i + 1 < INTR_MAX { qh_link(m, intr_qh(i + 1)) } else { LINK_T };
        init(m, intr_qh(i), next, chars(0, 0, 64, false), CAP_MULT_1);
    }
    for f in 0..FRAMES {
        m.wr(FRAME_LIST + 4 * f, qh_link(m, intr_qh(0)));
    }
}
