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
//! A driver killed at every step of a mode set, a flip and a cursor update
//! (G-72's sweeps, G-78), then restarted onto exactly what it left. Every
//! restart that finds the output lit takes it whole: the GGTT holds only
//! the new driver's entries and firmware's, nothing the engine reaches is
//! the killed driver's or a frame it gave back, and the new surface shows.
//! A mode set killed with the plane off and its pipe still running is
//! taken too; one killed with the transcoder off leaves nothing to
//! take, and then all the engine reaches is still the killed driver's own
//! frames (kept for good) or firmware's. The sweep needs both.

use super::death_sweep::sweep;
use crate::tests::deaths::{flip_deaths, mode_deaths};
use crate::tests::wire_rig::Rig;

#[test]
fn killed_at_any_step_the_restart_ends_whole() {
    let (w, _) = sweep(flip_deaths::flush, "a flip");
    assert!(w > 0);
    let (w, _) = sweep(flip_deaths::cursor, "a cursor update");
    assert!(w > 0);
    let set = |d: &mut Rig| {
        let _ = mode_deaths::set(d);
    };
    let (w, dark) = sweep(set, "a mode set");
    assert!(w > 0 && dark > 0, "{w} whole, {dark} dark");
}
