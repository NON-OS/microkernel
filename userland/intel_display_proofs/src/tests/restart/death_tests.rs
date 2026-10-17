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
//! A mode set killed with the plane off leaves nothing lit to take; then
//! all the engine reaches is still the killed driver's own frames (kept for
//! good) or firmware's. Those restarts are counted; the sweep needs both.

use super::death_kit::{rig, steps};
use super::kit::restart;
use super::restart_tests::reaches_only_ours;
use crate::model::confine::names;
use crate::tests::confine::kit::{confined, FIRMWARE};
use crate::tests::deaths::kit_held::held;
use crate::tests::deaths::kit::{run, Wrong};
use crate::tests::deaths::{flip_deaths, mode_deaths};
use crate::tests::wire_rig::Rig;

fn sweep(work: fn(&mut Rig), name: &str) -> (u32, u32) {
    let (mut whole, mut dark) = (0, 0);
    for k in steps(work) {
        let (mut d, model) = rig();
        run(&model, Wrong::Killed(k), || work(&mut d));
        let kept = held(&d);
        drop(d);
        let m = std::rc::Rc::try_unwrap(model).ok().expect("the dead driver gone").into_inner();
        let reach = m.reaching();
        let Some((_d, model)) = restart(m) else {
            // Nothing lit to take: what is reached is still the killed
            // driver's own frames, kept for good, or firmware's.
            let ok = |a: &u64| names(&kept, *a | 1) || names(&[FIRMWARE], *a | 1);
            assert!(reach.iter().all(ok), "{name} killed at {k}: dark, reaching {reach:x?}");
            dark += 1;
            continue;
        };
        let (m, tag) = (model.borrow(), format!("{name} killed at {k}"));
        confined(&m, &tag);
        reaches_only_ours(&m, &tag);
        assert_eq!(m.watch.reached, 0, "{tag}: a frame given back reached");
        let px = crate::engine::pipe::Pipe::ALL.iter().find_map(|&p| m.pixel(p, 5, 5));
        assert_eq!(px, Some(0xFF00_00AA), "{tag}: the new surface shows");
        whole += 1;
    }
    (whole, dark)
}

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
