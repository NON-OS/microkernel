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

//! SCANOUT: the compositor alone, on a driver with both scanout bits,
//! against the current primary, at its size; the band read from it.

use crate::model::wire::{ask, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use nonos_gfx_wire::body::caps::{BAND_PLANE, DIRECT_SCANOUT};
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, EPERM, ESTALE};
use nonos_gfx_wire::{ops, Rect, Scanout, Surface};

fn server(flags: u32) -> (Server<Vram, Pool>, u32) {
    let mut v = Vram::new(false, false);
    v.caps.flags |= flags;
    let mut s = Server::new(v, Pool::new(1024 * 768 * 4 + 16384));
    let (_, body) = ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    (s, Surface::decode(&body).expect("a primary").serial)
}

fn req(serial: u32) -> Scanout {
    let band = Rect { x: 1024 - 84, y: 0, width: 84, height: 16 };
    Scanout { handle: 0x77, serial, width: 1024, height: 768, stride: 4096, format: 1, band }
}

#[test]
fn the_compositor_puts_a_client_buffer_on_the_plane_with_the_band() {
    let (mut s, serial) = server(DIRECT_SCANOUT | BAND_PLANE);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::SCANOUT, 0, &req(serial).encode()).0, 0);
    let (handle, band) = s.display.scanned.clone().expect("scanned");
    assert_eq!(handle, 0x77);
    assert_eq!(band.len(), 84 * 16, "the band's pixels come from the primary");
}

#[test]
fn only_the_owner_on_a_driver_with_both_bits_against_the_current_primary() {
    let (mut s, serial) = server(DIRECT_SCANOUT | BAND_PLANE);
    let body = req(serial).encode();
    assert_eq!(ask(&mut s, COMPOSITOR + 1, ops::SCANOUT, 0, &body).0, EPERM);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::SCANOUT, 0, &req(serial + 1).encode()).0, ESTALE);
    let wrong = Scanout { width: 800, stride: 3200, ..req(serial) };
    assert_eq!(ask(&mut s, COMPOSITOR, ops::SCANOUT, 0, &wrong.encode()).0, EINVAL);
    assert!(s.display.scanned.is_none());
    for flags in [0, DIRECT_SCANOUT, BAND_PLANE] {
        let (mut s, serial) = server(flags);
        assert_eq!(ask(&mut s, COMPOSITOR, ops::SCANOUT, 0, &req(serial).encode()).0, ENOSYS);
    }
}

#[test]
fn a_body_that_does_not_fit_the_plane_is_refused_before_the_device() {
    let (mut s, serial) = server(DIRECT_SCANOUT | BAND_PLANE);
    let r = req(serial);
    let cases = [
        Scanout { handle: 0, ..r },
        Scanout { format: 2, ..r },
        Scanout { stride: 1024 * 4 - 4, ..r },
        Scanout { band: Rect { width: 129, ..r.band }, ..r },
        Scanout { band: Rect { x: 1000, ..r.band }, ..r },
        Scanout { band: Rect { height: 0, ..r.band }, ..r },
    ];
    for c in cases {
        assert_eq!(ask(&mut s, COMPOSITOR, ops::SCANOUT, 0, &c.encode()).0, EINVAL, "{c:?}");
    }
    assert_eq!(Scanout::decode(&r.encode()), Ok(r), "the body reads back as written");
    assert_eq!(ask(&mut s, COMPOSITOR, ops::SCANOUT, 0, &r.encode()[..43]).0, EINVAL);
}
