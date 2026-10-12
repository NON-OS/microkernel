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

//! The schedule every frame runs: each of the 1024 frame list entries
//! points at the first interrupt QH; the interrupt QHs chain to control's
//! QH, then bulk's, then the polled bulk IN's, which ends the frame. Every
//! queue is visited every frame, so an interrupt endpoint is polled each millisecond, at or
//! above any rate its bInterval asks for, and a NAK costs nothing but time.

use super::layout::*;
use super::qh::{init, qh_link};
use crate::env::Mem;

pub fn build<M: Mem>(m: &M) {
    for i in 0..INTR_MAX {
        let next = if i + 1 < INTR_MAX { intr_qh(i + 1) } else { CTRL_QH };
        init(m, intr_qh(i), Some(next));
    }
    init(m, CTRL_QH, Some(BULK_QH));
    init(m, BULK_QH, Some(POLL_QH));
    init(m, POLL_QH, None);
    let first = qh_link(m, INTR_QH);
    for f in 0..FRAMES {
        m.wr(FRAME_LIST + 4 * f, first);
    }
}
