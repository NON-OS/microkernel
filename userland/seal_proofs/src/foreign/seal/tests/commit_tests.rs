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

//! Hc as the kernel commits it, and what a seal says when guests outlive it.

use nonos_proof_abi::{hist_commit, HIST_WORDS};

use super::fixture::{family, SALT};
use crate::foreign::seal::finish::finish;
use crate::foreign::seal::hist::hist_commit as kernel_commit;
use crate::foreign::seal::table::LIVE;
#[test]
fn the_kernels_hc_is_the_verifiers_at_the_edges() {
    let mut h = [0u32; HIST_WORDS];
    assert_eq!(kernel_commit(&h, &SALT), hist_commit(&h, &SALT));
    h.iter_mut().enumerate().for_each(|(i, c)| *c = (i as u32).wrapping_mul(0x9E37_79B9));
    h[HIST_WORDS - 1] = u32::MAX;
    assert_eq!(kernel_commit(&h, &SALT), hist_commit(&h, &SALT));
    assert_eq!(kernel_commit(&h, &[u64::MAX, 0, 0, 0]), None);
}

#[test]
fn a_family_no_app_was_noted_in_is_dropped() {
    LIVE.lock().push(family(78));
    // finish answers the seal number of a record it keeps, so None is
    // nothing kept; SEALED's length is not compared, as the proofs running
    // beside this one add to that one table.
    assert_eq!(finish(78), None, "nothing was kept");
    assert!(LIVE.lock().iter().all(|f| f.sup != 78), "it is no longer live");
}
