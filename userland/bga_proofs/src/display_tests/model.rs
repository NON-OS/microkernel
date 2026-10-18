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

//! A driven adapter for the display tests. Its firmware left a small mode
//! showing, which the driver keeps, so a test can look at every pixel.

use nonos_libc::hold;

use crate::adapter::{adapter, found, Adapter, REG_LEN};
use crate::serve::{COMPOSITOR_SERVICE, OP_GET_PRIMARY_SURFACE};
use crate::setup::{run, Driver};

use super::frames::ask;
use super::left::showing;

/// The compositor's process, which the kernel registered its name to at
/// its spawn, and another one.
pub const COMPOSITOR: u32 = 0x27;
pub const OTHER: u32 = 0x31;

/// The driver first: it drops before the memory it writes into.
pub struct Served {
    pub d: Driver,
    pub a: Adapter,
}

/// An adapter showing `w` x `h`, BAR0 exactly that large, brought up, with
/// the compositor running.
pub fn served(w: u16, h: u16) -> Served {
    let bytes = w as usize * h as usize * 4;
    let a = adapter(REG_LEN, bytes);
    showing(&a, w, h);
    hold(COMPOSITOR_SERVICE, Some(COMPOSITOR));
    let d = run(found(bytes)).unwrap_or_else(|e| panic!("refused: {e:?}"));
    Served { d, a }
}

/// The compositor takes the primary, as its setup does first.
pub fn take(s: &mut Served) {
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[]).0, 0, "primary refused");
}

/// Every pixel of the surface drawn, each a value naming its index.
pub fn paint(s: &Served) {
    for i in 0..s.d.width as usize * s.d.height as usize {
        s.a.surface.present32(i * 4, drawn(i));
    }
}

pub fn drawn(i: usize) -> u32 {
    0x8000_0000 | i as u32
}
