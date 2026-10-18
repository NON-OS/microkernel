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

//! What a live family takes in: rows into K and H, measurements into M.

use super::family::{Family, MAX_MEAS};
use super::hist::{refusal, slot};
use super::meas::meas_step;
use super::trace::{trace_step, Row, ABANDONED, REFUSED, VALUE};

/*
 * The side a row lands on. Only an EPERM or ENOSYS answer, a call the kernel
 * turned away, or one whose park failed is a refusal; every other row counts
 * as a success. A mistake here can only fail an honest proof, never pass a
 * dishonest one.
 */
fn refused(row: &Row) -> bool {
    match row.what {
        VALUE | REFUSED => refusal(row.ret),
        ABANDONED => true,
        _ => false,
    }
}

impl Family {
    pub(crate) fn absorb(&mut self, row: &Row) {
        self.trace = trace_step(&self.trace, row);
        self.rows = self.rows.saturating_add(1);
        let s = slot(row.nr, refused(row));
        self.saturated |= self.hist[s] == u32::MAX;
        self.hist[s] = self.hist[s].saturating_add(1);
    }

    /// Each measurement once, in first-seen order, and M over exactly the
    /// list, so section 7 always rebuilds it.
    pub(crate) fn measured(&mut self, m: &[u8; 32]) {
        if self.overflow || self.list.contains(m) {
            return;
        }
        if self.list.len() >= MAX_MEAS {
            self.overflow = true;
            return;
        }
        self.meas = meas_step(&self.meas, m);
        self.n_meas += 1;
        self.list.push(*m);
    }

    pub(crate) fn stamp(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq = seq.wrapping_add(1);
        seq
    }
}
