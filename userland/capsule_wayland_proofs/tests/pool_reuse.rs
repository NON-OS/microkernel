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


//! A pool destroyed while a buffer is made from it keeps its memory, and its
//! id, told free, may name a new pool at once (libwayland gives it again):
//! the buffer goes on reading the old pool, not the new one.

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::linux::guest::memfd_spans::resolve;

const POOL_DESTROY: u16 = 1;
const OLD: u32 = 21;
const NEW: u32 = 22;

#[test]
fn a_buffer_keeps_its_pool_when_the_pool_id_is_given_again() {
    let mut c = Client::new();
    c.bind(REGISTRY, &[("wl_shm", 1, SHM)]);
    c.painted(SHM, POOL, OLD, (8, 8), 0xFFFF_0000);
    // Where the buffer's pixels are read: its pool's memfd, mapped now.
    let at = |c: &Client, b: u32| {
        let s = &c.s.guest.scene;
        let pool = s.buffers.iter().find(|x| x.id == b).expect("the buffer").pool;
        let memfd = s.pools.iter().find(|p| p.id == pool).map(|p| p.memfd)?;
        resolve(&c.s.guest.memfd_spans, memfd, 0, 1)
    };
    let before = at(&c, OLD).expect("its pool");
    c.req(POOL, POOL_DESTROY, &[]);
    c.painted(SHM, POOL, NEW, (8, 8), 0xFF00_00FF);
    c.events();
    assert!(!c.s.guest.objects.failed(), "the id was free");
    assert_eq!(at(&c, OLD), Some(before), "the old buffer reads its own pool");
    assert_ne!(at(&c, NEW), Some(before), "the new one reads the new pool");
}

#[test]
fn a_destroyed_pool_goes_with_its_last_buffer() {
    let mut c = Client::new();
    c.bind(REGISTRY, &[("wl_shm", 1, SHM)]);
    c.painted(SHM, POOL, OLD, (8, 8), 0xFFFF_0000);
    c.req(POOL, POOL_DESTROY, &[]);
    assert_eq!(c.s.guest.scene.pools.len(), 1, "kept for its buffer");
    c.req(OLD, 0, &[]);
    c.events();
    assert!(c.s.guest.scene.pools.is_empty(), "gone with it");
}

/// A create_pool too short to read still takes the descriptor it carried,
/// so the next create_pool is made over its own.
#[test]
fn a_short_create_pool_takes_its_descriptor_with_it() {
    let mut c = Client::new();
    c.bind(REGISTRY, &[("wl_shm", 1, SHM)]);
    // SAFETY: a fresh memfd for the test, handed over with the request.
    let stray = unsafe { libc::memfd_create(c"stray".as_ptr(), libc::MFD_CLOEXEC) };
    assert!(stray >= 0);
    c.req_fds(SHM, 0, &[A::U(POOL)], &[stray]);
    c.painted(SHM, 23, 24, (8, 8), 0xFF00_FF00);
    c.events();
    assert!(c.s.guest.scene.pools.iter().any(|p| p.id == 23), "made over its own descriptor");
    assert!(c.s.guest.scene.buffers.iter().any(|b| b.id == 24));
}

/// A buffer longer than MAX_SIDE on a side, or past its pool, is refused
/// with wl_shm's invalid_stride on the pool, and nothing is made.
#[test]
fn a_buffer_too_long_or_past_its_pool_is_invalid_stride() {
    for (w, h, stride) in [(1, 20_000, 4), (20_000, 1, 80_000), (8, 8, 16)] {
        let mut c = Client::new();
        c.bind(REGISTRY, &[("wl_shm", 1, SHM)]);
        let len = 4 << 20;
        // SAFETY: a fresh memfd sized for the pool, handed over with it.
        let fd = unsafe {
            let fd = libc::memfd_create(c"pool".as_ptr(), libc::MFD_CLOEXEC);
            assert!(fd >= 0 && libc::ftruncate(fd, len as libc::off_t) == 0);
            fd
        };
        c.req_fds(SHM, 0, &[A::U(POOL), A::I(len)], &[fd]);
        c.req(POOL, 0, &[A::U(OLD), A::I(0), A::I(w), A::I(h), A::I(stride), A::U(0)]);
        let evs = c.events();
        let err = to(&evs, 1).into_iter().find(|e| e.opcode == 0).map(|e| (e.word(0), e.word(1)));
        assert_eq!(err, Some((POOL, 2)), "{w}x{h} stride {stride}");
        assert!(c.s.guest.scene.buffers.is_empty());
    }
}
