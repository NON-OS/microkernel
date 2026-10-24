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

//! The wire sweep on a server whose primaries are the real pool's: every
//! frame a compositor might send, answered as the wire defines, and after
//! it no surface was ever registered holding a byte, none was unmapped but
//! whole, and the pages still mapped are exactly the primaries the server
//! still holds, each registered: nothing leaked, nothing freed under it.

use nonos_gfx_serve::Server;
use nonos_libc::{live, maps, registered_dirty, reset, set_compositor, unmapped_wrong};

use super::server_sweep::sweep;
use crate::tests::screen::Screen;
use crate::Pool;

#[test]
fn the_real_pool_holds_a_server_through_the_sweep() {
    reset();
    set_compositor(Some(7));
    let mut s = Server::new(Screen::new(), Pool::default());
    let t = sweep(&mut s, 0x6F01_0001, 20_000, 7);
    assert!(t.taken >= 2_000 && t.refused >= 2_000 && t.shown >= 100, "{t:?}");
    assert_eq!((registered_dirty(), unmapped_wrong()), (0, 0));
    let slots = s.slots.iter();
    let mut held: Vec<_> = slots.flat_map(|o| [o.current, o.retiring]).flatten().collect();
    held.sort_by_key(|p| p.region.base);
    let regions: Vec<_> = held.iter().map(|p| (p.region.base, p.region.len)).collect();
    assert_eq!(maps(), regions, "pages mapped that no primary holds, or the reverse");
    let mut named: Vec<_> = live().iter().map(|e| (e.base, e.len)).collect();
    named.sort();
    assert_eq!(named, regions, "a primary the kernel does not name, or the reverse");
}
