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
//! A mode set gone wrong at every command: the host dead from there, for a
//! while or for good, or the driver killed there. The scanout shows a whole
//! picture, old or new, never a surface half loaded (or nothing, the driver
//! Down, when the host never answers again); the host never reads pages the
//! driver gave back; a refused set shows the old mode, an answered one the
//! new; and with the host back the next sets leave one surface, the shown.

use nonos_gfx_wire::ops;

use super::ask::{ask, words};
use super::kit::{backed_live, draw, shown};
use super::rig::{rig, Rig};
use super::wrong::{run, wrongs, Wrong};
use crate::state::Health;

const OLD: u32 = 0x0011_2233;

fn set(r: &mut Rig, w: u32, h: u32) -> i32 {
    ask(r, ops::MODE_SET, 0, &words(&[w, h])).0
}

#[test]
fn a_mode_set_wrong_at_any_command_shows_a_whole_picture() {
    let n = {
        let mut r = rig();
        draw(&mut r, 0, OLD);
        let from = r.host.borrow().steps();
        set(&mut r, 800, 600);
        let to = r.host.borrow().steps();
        to - from
    };
    assert!(n >= 7, "{n} commands in a mode set");
    for w in wrongs(n) {
        let mut r = rig();
        draw(&mut r, 0, OLD);
        let got = run(&mut r, w, |r| set(r, 800, 600));
        if r.d.server.display.health == Health::Down {
            assert!(matches!(w, Wrong::Dead(_, 0)) && shown(&r, 0).is_none(), "{w:?}");
            continue;
        }
        let px = shown(&r, 0).expect("a scanout never left dark");
        let old = px.len() == 640 * 480 && px.iter().all(|&p| p == OLD);
        let new = px.len() == 800 * 600 && px.iter().all(|&p| p == 0);
        assert!(old || new, "{w:?} of {n}: a whole picture");
        assert!(backed_live(&r), "{w:?}: the host reads only pages held");
        let Wrong::Dead(_, d) = w else { continue };
        assert_eq!(got.unwrap() == 0, new, "{w:?}: the answer says what is shown");
        if d > 0 {
            if got == Some(0) {
                let serial = r.d.server.slots[0].retiring.unwrap().serial;
                assert_eq!(ask(&mut r, ops::RETIRE, 0, &words(&[serial])).0, 0);
            }
            let tries = (0..=d).take_while(|_| set(&mut r, 1024, 768) != 0).count() as u32;
            assert!(tries <= d, "{w:?}: set within the window");
            ask(&mut r, ops::STATE, 0, &[]);
            assert!(r.d.server.display.limbo.is_empty(), "{w:?}: nothing in limbo");
            assert_eq!(r.host.borrow().res.len(), 1, "{w:?}: one surface on the host");
        }
    }
}
