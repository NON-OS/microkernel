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
//! A scanout turned off and back with the host going wrong at any command
//! of it (G-77): dead for a while, for good, or the driver killed. The
//! host never reads a page the driver gave back, and with the host
//! answering again the primary is shown whole within the dead window's
//! tries, and only its surface is left on the host.

use nonos_gfx_wire::body::state::SCANNING_OUT;

use super::kit::{backed_live, draw, raise, shown, state};
use super::rig::{rig, Rig};
use super::wrong::{run, wrongs, Wrong};
use crate::tests::host::words::PANEL;

const PX: u32 = 0x0012_3456;

fn off_and_back(r: &mut Rig) {
    raise(r, 0, (0, 0));
    r.d.server.display.events();
    raise(r, 0, PANEL);
    r.d.server.display.events();
    state(r, 0);
}

#[test]
fn off_and_back_wrong_at_any_command_ends_whole() {
    let n = {
        let mut r = rig();
        draw(&mut r, 0, PX);
        let from = r.host.borrow().steps();
        off_and_back(&mut r);
        let to = r.host.borrow().steps();
        to - from
    };
    assert!(n >= 8, "{n} commands off and back");
    for w in wrongs(n) {
        let mut r = rig();
        draw(&mut r, 0, PX);
        let lived = run(&mut r, w, off_and_back).is_some();
        assert!(backed_live(&r), "{w:?}: the host reads only pages held");
        let Wrong::Dead(_, d) = w else { continue };
        assert!(lived, "{w:?}: a dead host kills nothing");
        if d == 0 {
            continue;
        }
        let tries = (0..=d + 1).take_while(|_| state(&mut r, 0).flags != SCANNING_OUT).count();
        assert!(tries as u32 <= d + 1, "{w:?}: scanning out within the window");
        let px = shown(&r, 0).expect("shown");
        assert!(px.len() == 640 * 480 && px.iter().all(|&p| p == PX), "{w:?}: whole");
        state(&mut r, 0);
        assert!(r.d.server.display.limbo.is_empty(), "{w:?}: nothing in limbo");
        assert_eq!(r.host.borrow().res.len(), 1, "{w:?}: one surface on the host");
    }
}
