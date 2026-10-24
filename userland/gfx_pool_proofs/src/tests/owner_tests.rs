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

//! A compositor started again (a new pid): rights die with their pid, so the
//! pool names it a writable receiver of each kept primary, handed on zero
//! after a silence; a handle the pool does not hold is shared with no one.

use nonos_gfx_serve::{Platform, Server, OWNER_SILENCE_MS};
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{ops, Surface};
use nonos_libc::{live, reset, set_compositor, surface_registry::SURFACE_SHARE_WRITE};

use super::screen::Screen;
use super::wire::ask;
use crate::Pool;

const FIRST: u32 = 0x27;
const AGAIN: u32 = 0x28;

fn surface(s: &mut Server<Screen, Pool>, from: u32) -> Surface {
    let (status, body) = ask(s, from, ops::SURFACE, &[]);
    assert_eq!(status, 0, "SURFACE from {from:#x}");
    Surface::decode(&body).unwrap()
}

#[test]
fn a_compositor_started_again_is_named_a_receiver_of_the_kept_primary() {
    reset();
    let mut s = Server::new(Screen::new(), Pool::default());
    let first = surface(&mut s, FIRST);
    set_compositor(Some(AGAIN));
    let again = surface(&mut s, AGAIN);
    assert_eq!(again.handle, first.handle, "the primary kept, not made again");
    assert!(live()[0].shares.contains(&(AGAIN, SURFACE_SHARE_WRITE)), "no right for the new pid");
}

#[test]
fn a_silent_owner_s_primary_reaches_the_next_one_zero() {
    reset();
    let mut s = Server::new(Screen::new(), Pool::default());
    surface(&mut s, FIRST);
    s.heard(FIRST, 0);
    let r = s.slots[0].current.unwrap().region;
    // SAFETY: the primary's pages are a live mapping the pool made, this long.
    unsafe { core::ptr::write_bytes(r.base as *mut u8, 0x5A, r.len) };
    assert!(s.watch(OWNER_SILENCE_MS));
    set_compositor(Some(AGAIN));
    surface(&mut s, AGAIN);
    assert!(s.platform.bytes(r).iter().all(|&b| b == 0), "the last owner's picture handed on");
}

#[test]
fn a_handle_given_back_or_never_made_is_shared_with_no_one() {
    reset();
    let mut pool = Pool::default();
    let r = pool.alloc(4096).unwrap();
    let handle = pool.register(r, 32, 32, 128).unwrap();
    assert_eq!(pool.share(handle ^ 1, AGAIN), Err(EINVAL));
    pool.release(Some(handle), r);
    assert_eq!(pool.share(handle, AGAIN), Err(EINVAL));
    assert!(live().is_empty());
}
