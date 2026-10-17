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

//! A mode set gone wrong at every step: the engine dead from there, for a
//! while or for good, or the driver killed there. No plane or cursor ever
//! reaches a frame the driver gave back; a killed driver leaves planes on
//! frames it held; a refused set keeps the old surfaces and mode.

use super::kit::{rig, run, Wrong};
use super::kit_held::{held, steps};
use crate::display::Plug;
use crate::tests::confine::kit::confined;
use crate::tests::wire::wire_modeset_tests::shadow_of;
use crate::tests::wire_rig::h0;
use nonos_gfx_serve::{Display, Shadow};

pub(crate) fn set(d: &mut crate::tests::wire_rig::Rig) -> Result<(), nonos_gfx_wire::Errno> {
    let px = shadow_of(0xFF13_5791, 1024, 768);
    let s = Shadow { bytes: &px, width: 1024, height: 768, stride: 4096, cursor_offset: 0 };
    d.set_mode(0, 1024, 768, &s)
}

#[test]
fn a_mode_set_wrong_at_any_step_never_shows_a_frame_given_back() {
    let at = steps(|d| set(d).unwrap());
    let (n, mut refused) = (at.len(), 0);
    for (i, &k) in at.iter().enumerate() {
        let brief = (i % 4 == 0).then_some(Wrong::Dead(k, 300));
        for w in [Wrong::Dead(k, 0), Wrong::Killed(k)].into_iter().chain(brief) {
            let (mut d, model) = rig();
            let old = (h0(&d).bases, h0(&d).scan.width, h0(&d).frames.iova);
            let got = run(&model, w, || set(&mut d));
            let m = model.borrow();
            assert_eq!(m.watch.reached, 0, "{w:?} of {n}: a plane on a frame given back");
            let held = held(&d);
            let on_held = |a: &u64| held.iter().any(|h| (h.0..h.1).contains(a));
            assert!(m.reaching().iter().all(on_held), "{w:?}: a plane off the frames held");
            confined(&m, &format!("{w:?} of {n}"));
            if let Some(Err(_)) = got {
                // Lit again when it could be; else left reaching nothing given back.
                let back = h0(&d).plug == Plug::In;
                assert!(!back || h0(&d).scan.width == old.1, "{w:?}: the old mode back");
                let f = &h0(&d).frames;
                assert_eq!((h0(&d).bases, f.iova, f.was), (old.0, old.2, None), "{w:?}: undone");
                refused += 1;
            }
        }
    }
    assert!(refused > 0, "{n} steps, none refused");
}
