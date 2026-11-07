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

//! Any bytes as one Wayland client's session against the shipped server:
//! every global bound at the version offered, then steps read from the input,
//! each a request to an object the client holds or never made, raw wire bytes
//! with any header, a shm descriptor, or input from the desktop. The server
//! must never panic, must answer with whole events, and must keep what it
//! holds bounded; the client's end is closed last, as the capsule's close does.
//! Each input starts from a fresh desktop and host world, on a clock that
//! moves 17 ms a step, and leaves no pool mapped, so a run is the same
//! however many came before it.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_capsule_wayland_proofs::client::{to, Client, A, REGISTRY};
use nonos_capsule_wayland_proofs::input;
use nonos_capsule_wayland_proofs::linux::{call, guest};

/// The most steps one input drives.
const STEPS: usize = 256;
/// The largest descriptor a step passes.
const MAX_FD: usize = 256 * 256 * 4;

/// The input read a byte or a word at a time, zero once it is spent.
struct Bytes<'a>(&'a [u8]);

impl Bytes<'_> {
    fn byte(&mut self) -> u8 {
        let Some((&b, rest)) = self.0.split_first() else { return 0 };
        self.0 = rest;
        b
    }
    fn word(&mut self) -> u32 {
        u32::from_le_bytes([self.byte(), self.byte(), self.byte(), self.byte()])
    }
    fn take(&mut self, n: usize) -> &[u8] {
        let (head, rest) = self.0.split_at(n.min(self.0.len()));
        self.0 = rest;
        head
    }
}

/// Every global the registry offers, bound at the version it offers, from
/// id 3 up; the ids the client holds.
fn bind_all(c: &mut Client) -> Vec<u32> {
    c.req(1, 1, &[A::U(REGISTRY)]);
    let mut ids = vec![1, REGISTRY];
    for e in to(&c.events(), REGISTRY).into_iter().filter(|e| e.opcode == 0) {
        let len = e.word(1) as usize;
        let iface = String::from_utf8_lossy(&e.body[8..8 + len - 1]).into_owned();
        let version = e.word(2 + len.div_ceil(4));
        let id = 1 + ids.len() as u32;
        c.req(REGISTRY, 0, &[A::U(e.word(0)), A::S(&iface), A::U(version), A::U(id)]);
        ids.push(id);
    }
    c.events();
    ids
}

/// A memfd of `len` bytes, owned by the session once sent.
fn memfd(len: usize) -> Option<i32> {
    // SAFETY: a fresh memfd, sized before it is handed to the session.
    unsafe {
        let fd = libc::memfd_create(c"fuzz".as_ptr(), libc::MFD_CLOEXEC);
        if fd < 0 {
            return None;
        }
        if libc::ftruncate(fd, len as libc::off_t) != 0 {
            libc::close(fd);
            return None;
        }
        Some(fd)
    }
}

/// One argument word: a held id, a fresh id, a small int, a size, or any.
fn arg(b: &mut Bytes, ids: &mut Vec<u32>, fresh: &mut u32) -> A<'static> {
    match b.byte() % 8 {
        0 | 1 => {
            *fresh += 1;
            ids.push(*fresh);
            A::U(*fresh)
        }
        2 | 3 => A::U(ids[b.byte() as usize % ids.len()]),
        4 => A::I(i32::from(b.byte() as i8)),
        5 => A::I(i32::from(u16::from_le_bytes([b.byte(), b.byte()]))),
        6 => A::S(["text/plain", "", "x", "image/png"][b.byte() as usize % 4]),
        _ => A::U(b.word()),
    }
}

fn step(c: &mut Client, b: &mut Bytes, ids: &mut Vec<u32>, fresh: &mut u32) {
    if c.s.guest.display.room() < 4096 {
        c.events();
    }
    match b.byte() % 10 {
        0 => input::tap(u32::from(b.byte() % 128), u16::from(b.byte() % 16)),
        1 => input::motion(i32::from(b.byte()) * 4 - 50, i32::from(b.byte()) * 3 - 50),
        2 => input::click(1 + u32::from(b.byte() % 3), (i32::from(b.byte()) * 4, i32::from(b.byte()) * 3)),
        3 => input::wheel(i32::from(b.byte() as i8), i32::from(b.byte() as i8), (100, 100)),
        4 => {
            let n = usize::from(b.byte()).min(c.s.guest.display.room());
            let raw = b.take(n).to_vec();
            c.s.send(&raw, &[]);
        }
        _ => {
            let object = match b.byte() % 8 {
                0 => u32::from(b.byte()),
                _ => ids[b.byte() as usize % ids.len()],
            };
            let opcode = u16::from(b.byte() % 16);
            let args: Vec<A> = (0..b.byte() % 8).map(|_| arg(b, ids, fresh)).collect();
            let fds: Vec<i32> = match b.byte() % 8 {
                0 => memfd(usize::from(u16::from_le_bytes([b.byte(), b.byte()])) * 4 % MAX_FD)
                    .into_iter()
                    .collect(),
                _ => Vec::new(),
            };
            c.req_fds(object, opcode, &args, &fds);
        }
    }
    if b.byte() % 3 == 0 {
        c.events();
    }
    let _ = c.s.tick();
    let s = &c.s.guest.scene;
    assert!(s.surfaces.len() <= 4096 && s.buffers.len() <= 4096 && s.pools.len() <= 4096);
    assert!(s.subs.restack.len() <= 256 && s.on_output.len() <= 4096);
}

fuzz_target!(|data: &[u8]| {
    nonos_app_skeleton::desk::with(|d| *d = Default::default());
    nonos_libc::world::with(|w| *w = Default::default());
    call::set_clock(1);
    let mut b = Bytes(data);
    let mut c = Client::new();
    let mut ids = bind_all(&mut c);
    let mut fresh = 100u32;
    for i in 0..STEPS as u64 {
        if b.0.is_empty() || c.s.guest.objects.failed() {
            break;
        }
        call::set_clock(1 + i * 17);
        step(&mut c, &mut b, &mut ids, &mut fresh);
    }
    c.events();
    c.s.disconnect();
    drop(c);
    guest::unmap_all();
});
