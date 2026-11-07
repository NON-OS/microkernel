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


//! The windows one client may have on screen: MOST_WINDOWS (32). A toplevel
//! past them is answered but not shown, and shows once one of the others
//! has gone.

use nonos_capsule_wayland_proofs::client::*;

const MOST_WINDOWS: usize = 32;

/// Toplevel `n`'s ids: surface, xdg_surface, toplevel, pool, buffer.
fn ids(n: u32) -> [u32; 5] {
    let b = 1000 + n * 10;
    [b, b + 1, b + 2, b + 3, b + 4]
}

fn open(c: &mut Client, n: u32) {
    let [surface, xdg, top, pool, buffer] = ids(n);
    c.req(COMPOSITOR, 0, &[A::U(surface)]);
    c.req(WM_BASE, 2, &[A::U(xdg), A::U(surface)]);
    c.req(xdg, 1, &[A::U(top)]);
    c.req(surface, 6, &[]);
    let serial = to(&c.events(), xdg)[0].word(0);
    c.req(xdg, 4, &[A::U(serial)]);
    c.buffer(SHM, pool, buffer, (32, 32));
    c.req(surface, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(surface, 6, &[]);
    c.events();
}

fn shown(c: &Client) -> usize {
    let s = &c.s.guest.scene;
    s.win.shown.is_some() as usize + s.behind.iter().filter(|w| w.shown.is_some()).count()
}

#[test]
fn a_client_shows_at_most_most_windows_and_the_next_when_one_goes() {
    let mut c = toplevel(6);
    for n in 0..40 {
        open(&mut c, n);
    }
    assert!(!c.s.guest.objects.failed(), "refused quietly, the client goes on");
    assert_eq!(shown(&c), MOST_WINDOWS);
    // One closes: the toplevel past the bound shows at its next commit.
    let [surface, xdg, top, ..] = ids(0);
    c.req(top, 0, &[]);
    c.req(xdg, 0, &[]);
    c.req(surface, 0, &[]);
    c.events();
    assert_eq!(shown(&c), MOST_WINDOWS - 1);
    let [late, ..] = ids(39);
    c.req(late, 6, &[]);
    c.events();
    assert_eq!(shown(&c), MOST_WINDOWS);
}
