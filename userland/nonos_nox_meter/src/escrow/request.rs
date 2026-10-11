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

//! The meter's FETCH 1 ESCROW request (plan/29) in one go: one fresh buyer
//! key, a nonce start, and the walk (walk.rs) run to its end. The capsule
//! runs the same walk in slices; the tests use this.

use nonos_equix::{Solver, INDEX_SPACE};
use nonos_nox_chunks::X25519;

use super::walk::Walk;
use super::walk_found::Walked;
use super::want::{Asked, Want};

/// The request, or None when `draw` gives out or no nonce meets the effort.
pub fn escrow_request(
    x: &impl X25519,
    draw: &mut impl FnMut() -> Option<[u8; 32]>,
    solver: &mut Solver,
    want: &Want,
) -> Option<Asked> {
    let (buyer_sk, start) = (draw()?, draw()?);
    let mut walk = Walk::new(x, buyer_sk, start[..16].try_into().ok()?, want.clone());
    loop {
        match walk.step(solver, INDEX_SPACE) {
            Walked::Going => {}
            Walked::Found(a) => return Some(a),
            Walked::GaveUp => return None,
        }
    }
}
