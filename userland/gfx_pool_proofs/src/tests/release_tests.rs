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

//! A primary given back: its surface withdrawn first, then exactly its
//! pages unmapped, whole; one never registered unmapped alone. Nothing else
//! is touched.

use nonos_gfx_serve::Platform;
use nonos_libc::{live, maps, reset, unmapped_wrong};

use crate::Pool;

#[test]
fn a_primary_given_back_withdraws_its_surface_then_unmaps_its_pages() {
    reset();
    let mut pool = Pool::default();
    let (a, b) = (pool.alloc(8192).unwrap(), pool.alloc(4096).unwrap());
    let (ha, hb) = (pool.register(a, 32, 64, 128).unwrap(), pool.register(b, 32, 32, 128).unwrap());
    pool.release(Some(ha), a);
    assert_eq!(live().iter().map(|e| e.handle).collect::<Vec<_>>(), [hb]);
    assert_eq!(maps(), [(b.base, b.len)]);
    assert_eq!(unmapped_wrong(), 0, "pages unmapped that were not one whole mapping");
}

#[test]
fn a_primary_never_registered_is_unmapped_alone() {
    reset();
    let mut pool = Pool::default();
    let (a, b) = (pool.alloc(4096).unwrap(), pool.alloc(4096).unwrap());
    let hb = pool.register(b, 32, 32, 128).unwrap();
    pool.release(None, a);
    assert_eq!((maps(), live().len()), (vec![(b.base, b.len)], 1));
    assert_eq!(live()[0].handle, hb);
    assert_eq!(unmapped_wrong(), 0);
}
