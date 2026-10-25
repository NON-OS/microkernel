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

//! A primary published: registered zero, and shared writable with the
//! compositor alone. Every refusal on the way leaves nothing registered: a
//! surface registered but not shared used to stay in the kernel's registry,
//! naming frames the server then freed, with no handle to let it go by.

use nonos_gfx_serve::Platform;
use nonos_gfx_wire::errno::EINVAL;
use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;
use nonos_libc::{live, refuse_registers, refuse_shares, reset, set_compositor};

use crate::Pool;

#[test]
fn a_primary_is_registered_zero_and_shared_with_the_compositor_alone() {
    reset();
    let mut pool = Pool::default();
    let r = pool.alloc(640 * 480 * 4).unwrap();
    let handle = pool.register(r, 640, 480, 2560).expect("published");
    let e = &live()[..];
    assert_eq!(e.len(), 1);
    assert_eq!((e[0].handle, e[0].base, e[0].len), (handle, r.base, r.len));
    assert!(e[0].zero, "registered with bytes in it");
    assert_eq!(e[0].shares, [(0x27, SURFACE_SHARE_WRITE)], "shared with anyone but it");
    assert_eq!(pool.share(handle, 0x27), Ok(()));
    assert_eq!(live()[0].shares.len(), 1, "named twice, one right");
}

#[test]
fn no_compositor_or_a_refused_register_registers_nothing() {
    reset();
    let mut pool = Pool::default();
    let r = pool.alloc(4096).unwrap();
    set_compositor(None);
    assert_eq!(pool.register(r, 32, 32, 128), Err(EINVAL));
    set_compositor(Some(0x27));
    refuse_registers(true);
    assert_eq!(pool.register(r, 32, 32, 128), Err(EINVAL));
    assert!(live().is_empty());
}

#[test]
fn a_share_refused_after_the_register_gives_the_surface_back() {
    reset();
    let mut pool = Pool::default();
    let r = pool.alloc(4096).unwrap();
    refuse_shares(true);
    assert_eq!(pool.register(r, 32, 32, 128), Err(EINVAL));
    assert!(live().is_empty(), "the kernel still names the frames: {:?}", live());
}

#[test]
fn a_caller_that_is_not_the_compositor_is_never_named_a_receiver() {
    reset();
    let mut pool = Pool::default();
    let r = pool.alloc(4096).unwrap();
    let handle = pool.register(r, 32, 32, 128).unwrap();
    assert_eq!(pool.share(handle, 0x31), Err(EINVAL));
    assert_eq!(live()[0].shares, [(0x27, SURFACE_SHARE_WRITE)], "a right for another pid");
}
