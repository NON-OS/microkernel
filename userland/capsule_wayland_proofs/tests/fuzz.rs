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

//! Requests a client should never send (fuzz_protocol.rs sends the ones it
//! may, to reach deep states), thrown at the server in long seeded
//! runs: every global bound, then messages to any object it holds or never
//! made, with ids it holds, fresh ones and nonsense, strings, descriptors,
//! and input between. The server is the shipped files; it must never panic,
//! must answer each batch with whole events, and must keep what it holds
//! bounded however the client behaves.

use std::panic::{catch_unwind, AssertUnwindSafe};

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input;

const RUNS: u64 = 300;
const STEPS: usize = 400;

/// RUNS, or NONOS_FUZZ_RUNS for a longer search.
fn runs() -> u64 {
    std::env::var("NONOS_FUZZ_RUNS").ok().and_then(|v| v.parse().ok()).unwrap_or(RUNS)
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
}

/// Every global the registry offers, bound at the version it offers, from
/// id 3 up; the ids bound.
fn bind_all(c: &mut Client) -> Vec<u32> {
    c.req(1, 1, &[A::U(REGISTRY)]);
    let globals: Vec<(u32, String, u32)> = to(&c.events(), REGISTRY)
        .into_iter()
        .filter(|e| e.opcode == 0)
        .map(|e| {
            let len = e.word(1) as usize;
            let name = String::from_utf8_lossy(&e.body[8..8 + len - 1]).into_owned();
            let at = 8 + ((len + 3) & !3);
            let version = u32::from_le_bytes(e.body[at..at + 4].try_into().expect("a word"));
            (e.word(0), name, version)
        })
        .collect();
    let mut ids = vec![1, REGISTRY];
    for (i, (name, iface, version)) in globals.iter().enumerate() {
        let id = 3 + i as u32;
        c.req(REGISTRY, 0, &[A::U(*name), A::S(iface), A::U(*version), A::U(id)]);
        ids.push(id);
    }
    c.events();
    ids
}

fn memfd(len: usize) -> i32 {
    // SAFETY: a fresh memfd, sized, handed to the session with a request.
    unsafe {
        let fd = libc::memfd_create(c"fuzz".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, len as libc::off_t) == 0);
        fd
    }
}

/// One run: the requests that reached a live client, or why it failed and
/// the last messages sent.
fn run(seed: u64) -> Result<usize, String> {
    let mut log: Vec<String> = Vec::new();
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        let mut r = Rng(seed | 1);
        let mut c = Client::new();
        let mut ids = bind_all(&mut c);
        let mut fresh = 100u32;
        let mut served = 0;
        for _ in 0..STEPS {
            // A client ended for its error is gone; the next one starts over.
            if c.s.guest.objects.failed() {
                c = Client::new();
                ids = bind_all(&mut c);
                fresh = 100;
            }
            served += 1;
            match r.below(10) {
                0 => input::tap(r.below(128) as u32, r.below(16) as u16),
                1 => input::motion(r.below(1000) as i32 - 50, r.below(800) as i32 - 50),
                2 => {
                    input::click(1 + r.below(3) as u32, (r.below(900) as i32, r.below(700) as i32))
                }
                _ => {}
            }
            let object = match r.below(8) {
                0 => r.below(300) as u32,
                _ => ids[r.below(ids.len() as u64) as usize],
            };
            let opcode = r.below(14) as u16;
            let mut args = Vec::new();
            for _ in 0..r.below(7) {
                args.push(match r.below(9) {
                    0..=2 => {
                        fresh += 1;
                        ids.push(fresh);
                        A::U(fresh)
                    }
                    3..=4 => A::U(ids[r.below(ids.len() as u64) as usize]),
                    5 => A::I(r.below(4096) as i32 - 64),
                    6 => A::U(r.next() as u32),
                    7 => A::S(
                        ["text/plain", "", "x", "wl_surface", "image/png"][r.below(5) as usize],
                    ),
                    _ => A::U(0),
                });
            }
            let fds: Vec<i32> = match r.below(12) {
                0 => vec![memfd(64 * 64 * 4)],
                _ => Vec::new(),
            };
            log.push(format!("{object}.{opcode} {} args {} fds", args.len(), fds.len()));
            if log.len() > 12 {
                log.remove(0);
            }
            if c.s.guest.display.room() < 4096 {
                c.events();
            }
            c.req_fds(object, opcode, &args, &fds);
            if r.below(3) == 0 {
                c.events();
            }
            let _ = c.s.tick();
            let s = &c.s.guest.scene;
            assert!(s.surfaces.len() <= 4096 && s.buffers.len() <= 4096 && s.pools.len() <= 4096);
            assert!(s.subs.restack.len() <= 256 && s.on_output.len() <= 4096);
        }
        c.events();
        served
    }));
    outcome.map_err(|e| {
        let why = e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        format!("seed {seed}: {why}\n  last: {}", log.join(" | "))
    })
}

#[test]
fn nothing_a_client_sends_panics_the_server_or_grows_it_without_bound() {
    let results: Vec<Result<usize, String>> =
        (1..=runs()).map(|seed| run(seed * 0x9E37_79B9)).collect();
    let served: usize = results.iter().filter_map(|r| r.as_ref().ok()).sum();
    let failed: Vec<&String> = results.iter().filter_map(|r| r.as_ref().err()).collect();
    eprintln!("[fuzz] {served} requests over {} runs", runs());
    assert!(
        failed.is_empty(),
        "{} of {} runs failed:\n{}",
        failed.len(),
        runs(),
        failed.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n")
    );
}
