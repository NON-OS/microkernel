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

//! The lists the controller walks, built before it runs: every HCCA
//! interrupt entry points at the first interrupt ED, so every interrupt ED
//! is visited each frame (at or above any rate its bInterval asks); the
//! control list holds the one control ED, the bulk list every bulk ED.
//! Each ED starts skipped, its queue empty on its first TD place.

use super::ed::init;
use super::layout::*;
use crate::env::Mem;

pub fn build<M: Mem>(m: &M) {
    for o in (HCCA..HCCA + HCCA_LEN).step_by(4) {
        m.wr(o, 0);
    }
    for i in 0..INTR_MAX {
        let next = if i + 1 < INTR_MAX { m.bus(intr_ed(i + 1)) } else { 0 };
        init(m, intr_ed(i), intr_td(i, 0), next);
    }
    for f in 0..32 {
        m.wr(HCCA + 4 * f, m.bus(intr_ed(0)));
    }
    init(m, CTRL_ED, ctrl_td(0), 0);
    for i in 0..BULK_EDS {
        let next = if i + 1 < BULK_EDS { m.bus(bulk_ed(i + 1)) } else { 0 };
        init(m, bulk_ed(i), bulk_td(i, 0), next);
    }
}
