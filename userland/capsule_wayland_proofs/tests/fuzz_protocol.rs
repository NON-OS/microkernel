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

//! The protocol's own requests, in seeded runs: each to an object the
//! client holds, of that object's interface, with its arguments' shapes,
//! serials the server sent, buffers in real pools, and input between. Deep
//! states (windows shown, popups grabbed, subsurfaces, drags, selections)
//! are reached and torn down in every order. The server must never panic,
//! must send whole events, and must keep what it holds bounded.

use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input;

const RUNS: u64 = 400;
const STEPS: usize = 500;

/// Each interface's requests: opcode, the argument shapes, and the interface
/// of the object its new id makes. n new id, o any held id, s a surface,
/// b a buffer, p a positioner, x an xdg_surface, e a seat, u a serial or
/// small number, i an int, w a width or height, S a mime string, t a title,
/// f a descriptor.
type Requests = &'static [(u16, &'static str, &'static str)];

const SPEC: &[(&str, Requests)] = &[
    ("wl_compositor", &[(0, "n", "wl_surface"), (1, "n", "wl_region")]),
    ("wl_shm", &[(0, "nfP", "wl_shm_pool")]),
    ("wl_shm_pool", &[(0, "nB", "wl_buffer"), (0, "nB", "wl_buffer"), (1, "", ""), (2, "P", "")]),
    ("wl_buffer", &[(0, "", "")]),
    (
        "wl_surface",
        &[
            (0, "", ""),
            (1, "bii", ""),
            (2, "iiww", ""),
            (3, "n", "wl_callback"),
            (4, "o", ""),
            (5, "o", ""),
            (6, "", ""),
            (6, "", ""),
            (6, "", ""),
            (7, "u", ""),
            (8, "u", ""),
            (9, "iiww", ""),
            (10, "ii", ""),
        ],
    ),
    ("wl_region", &[(0, "", ""), (1, "iiww", ""), (2, "iiww", "")]),
    ("xdg_wm_base", &[(1, "n", "xdg_positioner"), (2, "ns", "xdg_surface"), (3, "u", "")]),
    (
        "xdg_positioner",
        &[
            (0, "", ""),
            (1, "ww", ""),
            (2, "iiww", ""),
            (3, "u", ""),
            (4, "u", ""),
            (5, "u", ""),
            (6, "ii", ""),
            (7, "", ""),
            (8, "ww", ""),
            (9, "u", ""),
        ],
    ),
    (
        "xdg_surface",
        &[
            (0, "", ""),
            (1, "n", "xdg_toplevel"),
            (2, "nxp", "xdg_popup"),
            (3, "iiww", ""),
            (4, "u", ""),
            (4, "u", ""),
        ],
    ),
    (
        "xdg_toplevel",
        &[
            (0, "", ""),
            (1, "o", ""),
            (2, "t", ""),
            (3, "t", ""),
            (4, "euii", ""),
            (5, "eu", ""),
            (6, "euu", ""),
            (7, "ww", ""),
            (8, "ww", ""),
            (9, "", ""),
            (10, "", ""),
            (11, "o", ""),
            (12, "", ""),
            (13, "", ""),
        ],
    ),
    ("xdg_popup", &[(0, "", ""), (1, "eu", ""), (2, "pu", "")]),
    (
        "wl_seat",
        &[(0, "n", "wl_pointer"), (1, "n", "wl_keyboard"), (2, "n", "wl_touch"), (3, "", "")],
    ),
    ("wl_pointer", &[(0, "usii", ""), (1, "", "")]),
    ("wl_keyboard", &[(0, "", "")]),
    ("wl_subcompositor", &[(0, "", ""), (1, "nss", "wl_subsurface")]),
    (
        "wl_subsurface",
        &[(0, "", ""), (1, "ii", ""), (2, "s", ""), (3, "s", ""), (4, "", ""), (5, "", "")],
    ),
    ("wl_data_device_manager", &[(0, "n", "wl_data_source"), (1, "ne", "wl_data_device")]),
    ("wl_data_source", &[(0, "S", ""), (1, "", ""), (2, "u", "")]),
    ("wl_data_device", &[(0, "osou", ""), (1, "ou", ""), (2, "", "")]),
    ("wl_data_offer", &[(0, "uS", ""), (1, "Sf", ""), (2, "", ""), (3, "", ""), (4, "uu", "")]),
    ("zxdg_decoration_manager_v1", &[(0, "", ""), (1, "no", "zxdg_toplevel_decoration_v1")]),
    ("zxdg_toplevel_decoration_v1", &[(0, "", ""), (1, "u", ""), (2, "", "")]),
    ("wp_viewporter", &[(0, "", ""), (1, "ns", "wp_viewport")]),
    ("wp_viewport", &[(0, "", ""), (1, "iiww", ""), (2, "ww", "")]),
    ("wp_fractional_scale_manager_v1", &[(0, "", ""), (1, "ns", "wp_fractional_scale_v1")]),
    ("wp_presentation", &[(0, "", ""), (1, "sn", "wp_presentation_feedback")]),
    ("wp_single_pixel_buffer_manager_v1", &[(0, "", ""), (1, "nuuuu", "wl_buffer")]),
    ("wp_cursor_shape_manager_v1", &[(0, "", ""), (1, "no", "wp_cursor_shape_device_v1")]),
    ("wp_cursor_shape_device_v1", &[(0, "", ""), (1, "uu", "")]),
    (
        "zwp_primary_selection_device_manager_v1",
        &[
            (0, "n", "zwp_primary_selection_source_v1"),
            (1, "ne", "zwp_primary_selection_device_v1"),
            (2, "", ""),
        ],
    ),
    ("zwp_primary_selection_source_v1", &[(0, "S", ""), (1, "", "")]),
    ("zwp_primary_selection_device_v1", &[(0, "ou", ""), (1, "", "")]),
    ("org_kde_kwin_server_decoration_manager", &[(0, "ns", "org_kde_kwin_server_decoration")]),
    ("org_kde_kwin_server_decoration", &[(0, "", ""), (1, "u", "")]),
    ("xdg_activation_v1", &[(0, "", ""), (1, "n", "xdg_activation_token_v1"), (2, "ts", "")]),
    ("zxdg_output_manager_v1", &[(0, "", ""), (1, "no", "zxdg_output_v1")]),
];

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

thread_local! {
    /// Steps with a window shown, a buffer, a popup, a toplevel, a subsurface.
    static DEPTH: std::cell::RefCell<[usize; 5]> = const { std::cell::RefCell::new([0; 5]) };
}

struct Fuzz {
    c: Client,
    kinds: HashMap<u32, &'static str>,
    serials: Vec<u32>,
    next: u32,
}

impl Fuzz {
    fn new() -> Fuzz {
        let mut f =
            Fuzz { c: Client::new(), kinds: HashMap::new(), serials: vec![0, 1], next: 200 };
        f.c.req(1, 1, &[A::U(REGISTRY)]);
        let globals: Vec<(u32, String, u32)> = to(&f.c.events(), REGISTRY)
            .into_iter()
            .filter(|e| e.opcode == 0)
            .map(|e| {
                let len = e.word(1) as usize;
                let name = String::from_utf8_lossy(&e.body[8..8 + len - 1]).into_owned();
                let at = 8 + ((len + 3) & !3);
                (
                    e.word(0),
                    name,
                    u32::from_le_bytes(e.body[at..at + 4].try_into().expect("a word")),
                )
            })
            .collect();
        for (i, (name, iface, version)) in globals.iter().enumerate() {
            let id = 3 + i as u32;
            f.c.req(REGISTRY, 0, &[A::U(*name), A::S(iface), A::U(*version), A::U(id)]);
            if let Some(&(known, _)) = SPEC.iter().find(|(n, _)| *n == iface.as_str()) {
                f.kinds.insert(id, known);
            }
        }
        f.read();
        f
    }

    fn read(&mut self) {
        for e in self.c.events() {
            // Serials a request may name: the first words of what it was sent.
            if !e.body.is_empty() && e.body.len() >= 4 {
                self.serials.push(e.word(0));
            }
            // Offers the server made are the client's to use.
            if e.opcode == 0 && self.kinds.get(&e.object) == Some(&"wl_data_device") {
                self.kinds.insert(e.word(0), "wl_data_offer");
            }
        }
        if self.serials.len() > 64 {
            self.serials.drain(..32);
        }
        let held = &self.c.s.guest.objects;
        self.kinds.retain(|id, _| held.get(*id).is_some());
    }

    fn held(&mut self, r: &mut Rng, iface: &str) -> u32 {
        let ids: Vec<u32> =
            self.kinds.iter().filter(|(_, k)| **k == iface).map(|(i, _)| *i).collect();
        match ids.is_empty() {
            true => r.below(400) as u32,
            false => ids[r.below(ids.len() as u64) as usize],
        }
    }

    /// A window made as a client makes one: a surface, its xdg_surface and
    /// toplevel, the initial commit, the configure acked, a buffer committed.
    fn window(&mut self, r: &mut Rng) {
        let (Some(comp), Some(wm), Some(shm)) =
            (self.any("wl_compositor"), self.any("xdg_wm_base"), self.any("wl_shm"))
        else {
            return;
        };
        let ids: Vec<u32> = (1..=6).map(|k| self.next + k).collect();
        self.next += 6;
        let [surface, xdg, top, pool, buffer, _] = ids[..] else { return };
        self.c.req(comp, 0, &[A::U(surface)]);
        self.c.req(wm, 2, &[A::U(xdg), A::U(surface)]);
        self.c.req(xdg, 1, &[A::U(top)]);
        self.c.req(surface, 6, &[]);
        let evs = self.c.events();
        let Some(serial) = to(&evs, xdg).first().map(|e| e.word(0)) else { return };
        self.c.req(xdg, 4, &[A::U(serial)]);
        let (w, h) = (16 + r.below(200) as i32, 16 + r.below(200) as i32);
        self.c.req_fds(shm, 0, &[A::U(pool), A::I(w * h * 4)], &[memfd((w * h * 4) as usize)]);
        self.c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(w), A::I(h), A::I(w * 4), A::U(0)]);
        self.c.req(surface, 1, &[A::U(buffer), A::I(0), A::I(0)]);
        self.c.req(surface, 6, &[]);
        for (id, k) in [
            (surface, "wl_surface"),
            (xdg, "xdg_surface"),
            (top, "xdg_toplevel"),
            (pool, "wl_shm_pool"),
            (buffer, "wl_buffer"),
        ] {
            self.kinds.insert(id, k);
        }
        self.read();
        if r.below(2) == 0 {
            self.popup(r, xdg, shm);
        }
    }

    /// A popup on xdg_surface `parent`, placed by a whole positioner, drawn,
    /// and given a grab with a serial the server sent.
    fn popup(&mut self, r: &mut Rng, parent: u32, shm: u32) {
        let (Some(comp), Some(wm)) = (self.any("wl_compositor"), self.any("xdg_wm_base")) else {
            return;
        };
        let ids: Vec<u32> = (1..=6).map(|k| self.next + k).collect();
        self.next += 6;
        let [pos, surface, xdg, popup, pool, buffer] = ids[..] else { return };
        let (w, h) = (8 + r.below(300) as i32, 8 + r.below(300) as i32);
        self.c.req(wm, 1, &[A::U(pos)]);
        self.c.req(pos, 1, &[A::I(w), A::I(h)]);
        self.c.req(
            pos,
            2,
            &[A::I(r.below(200) as i32), A::I(r.below(200) as i32), A::I(1), A::I(1)],
        );
        self.c.req(pos, 5, &[A::U(r.below(64) as u32)]);
        self.c.req(comp, 0, &[A::U(surface)]);
        self.c.req(wm, 2, &[A::U(xdg), A::U(surface)]);
        self.c.req(xdg, 2, &[A::U(popup), A::U(parent), A::U(pos)]);
        if r.below(2) == 0 {
            let seat = self.any("wl_seat").unwrap_or(0);
            let serial = self.serials[r.below(self.serials.len() as u64) as usize];
            self.c.req(popup, 1, &[A::U(seat), A::U(serial)]);
        }
        self.c.req(surface, 6, &[]);
        let evs = self.c.events();
        if let Some(serial) = to(&evs, xdg).first().map(|e| e.word(0)) {
            self.c.req(xdg, 4, &[A::U(serial)]);
        }
        self.c.req_fds(shm, 0, &[A::U(pool), A::I(w * h * 4)], &[memfd((w * h * 4) as usize)]);
        self.c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(w), A::I(h), A::I(w * 4), A::U(0)]);
        self.c.req(surface, 1, &[A::U(buffer), A::I(0), A::I(0)]);
        self.c.req(surface, 6, &[]);
        for (id, k) in [
            (pos, "xdg_positioner"),
            (surface, "wl_surface"),
            (xdg, "xdg_surface"),
            (popup, "xdg_popup"),
            (pool, "wl_shm_pool"),
            (buffer, "wl_buffer"),
        ] {
            self.kinds.insert(id, k);
        }
        self.read();
    }

    fn any(&self, iface: &str) -> Option<u32> {
        self.kinds.iter().find(|(_, k)| **k == iface).map(|(i, _)| *i)
    }

    fn step(&mut self, r: &mut Rng) -> String {
        if r.below(40) == 0 {
            self.window(r);
            return String::from("window");
        }
        match r.below(12) {
            0 => input::tap(r.below(128) as u32, r.below(4) as u16),
            1 => input::motion(r.below(1000) as i32, r.below(800) as i32),
            2 => input::click(1 + r.below(3) as u32, (r.below(1000) as i32, r.below(800) as i32)),
            3 => input::wheel(0, r.below(5) as i32 - 2, (300, 300)),
            _ => {}
        }
        let ids: Vec<u32> = self.kinds.keys().copied().collect();
        let object = ids[r.below(ids.len() as u64) as usize];
        let iface = self.kinds[&object];
        let Some(&(_, reqs)) = SPEC.iter().find(|(n, _)| *n == iface) else { return String::new() };
        let &(opcode, shape, makes) = &reqs[r.below(reqs.len() as u64) as usize];
        let mut words: Vec<A> = Vec::new();
        let mut fds = Vec::new();
        let mut made = None;
        for ch in shape.chars() {
            if ch == 'B' {
                // A buffer in its pool: offset 0, a stride its width fills, as
                // clients make them, now and then one that does not fit.
                let (w, h) = (1 + r.below(256) as i32, 1 + r.below(256) as i32);
                let stride = match r.below(8) {
                    0 => w * 4 - 4,
                    1 => r.next() as i32,
                    _ => w * 4,
                };
                let offset = match r.below(8) {
                    0 => r.below(1 << 20) as i32,
                    _ => 0,
                };
                for a in [offset, w, h, stride, r.below(2) as i32] {
                    words.push(A::I(a));
                }
                continue;
            }
            words.push(match ch {
                'n' => {
                    self.next += 1;
                    made = Some(self.next);
                    A::U(self.next)
                }
                'o' => match r.below(4) {
                    0 => A::U(0),
                    _ => A::U(ids[r.below(ids.len() as u64) as usize]),
                },
                's' => A::U(self.held(r, "wl_surface")),
                'b' => match r.below(6) {
                    0 => A::U(0),
                    _ => A::U(self.held(r, "wl_buffer")),
                },
                'p' => A::U(self.held(r, "xdg_positioner")),
                'x' => A::U(self.held(r, "xdg_surface")),
                'e' => A::U(self.held(r, "wl_seat")),
                'u' => A::U(match r.below(3) {
                    0 => r.below(8) as u32,
                    _ => self.serials[r.below(self.serials.len() as u64) as usize],
                }),
                'i' => A::I(r.below(600) as i32 - 100),
                'w' => A::I(match r.below(8) {
                    0 => 0,
                    1 => r.next() as i32,
                    _ => 1 + r.below(256) as i32,
                }),
                'P' => A::I(match r.below(10) {
                    0 => r.next() as i32,
                    1 => 0,
                    _ => 256 * 256 * 4,
                }),
                'S' => A::S(
                    ["text/plain;charset=utf-8", "text/plain", "image/png", ""]
                        [r.below(4) as usize],
                ),
                't' => A::S(["", "a window", "ünïcödé"][r.below(3) as usize]),
                'f' => {
                    fds.push(memfd(256 * 256 * 4));
                    continue;
                }
                _ => A::U(0),
            });
        }
        // A descriptor goes in control data, not the body.
        let body = words;
        if self.c.s.guest.display.room() < 8192 {
            self.read();
        }
        self.c.req_fds(object, opcode, &body, &fds);
        if let (Some(id), false) = (made, makes.is_empty()) {
            let k = SPEC.iter().find(|(n, _)| *n == makes).map(|(n, _)| *n).unwrap_or("other");
            self.kinds.insert(id, k);
        }
        if r.below(3) == 0 {
            self.read();
        }
        let _ = self.c.s.tick();
        format!("{iface}@{object}.{opcode}")
    }
}

fn memfd(len: usize) -> i32 {
    // SAFETY: a fresh memfd, sized, handed to the session with a request.
    unsafe {
        let fd = libc::memfd_create(c"fuzz".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, len as libc::off_t) == 0);
        fd
    }
}

fn run(seed: u64) -> Result<usize, String> {
    let mut log: Vec<String> = Vec::new();
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        let mut r = Rng(seed | 1);
        let mut f = Fuzz::new();
        let mut served = 0;
        for _ in 0..STEPS {
            if f.c.s.guest.objects.failed() {
                f = Fuzz::new();
            }
            log.push(f.step(&mut r));
            if log.len() > 16 {
                log.remove(0);
            }
            served += 1;
            let s = &f.c.s.guest.scene;
            DEPTH.with(|d| {
                let mut d = d.borrow_mut();
                d[0] += s.win.shown.is_some() as usize;
                d[1] += !s.buffers.is_empty() as usize;
                d[2] += !s.popups.list.is_empty() as usize;
                d[3] += s.surfaces.iter().any(|x| x.toplevel.is_some()) as usize;
                d[4] += !s.subs.list.is_empty() as usize;
            });
            assert!(s.surfaces.len() <= 4096 && s.buffers.len() <= 4096 && s.pools.len() <= 4096);
            assert!(s.subs.restack.len() <= 256 && s.on_output.len() <= 4096);
            assert!(s.behind.len() <= 4096 && s.popups.list.len() <= 4096);
        }
        f.read();
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
fn the_protocol_s_own_requests_in_any_order_never_panic_the_server() {
    let results: Vec<Result<usize, String>> = (1..=runs()).map(|s| run(s * 0x9E37_79B9)).collect();
    let served: usize = results.iter().filter_map(|r| r.as_ref().ok()).sum();
    let failed: Vec<&String> = results.iter().filter_map(|r| r.as_ref().err()).collect();
    let depth = DEPTH.with(|d| *d.borrow());
    eprintln!(
        "[fuzz] {served} requests, {} runs; steps shown/buffer/popup/toplevel/sub {depth:?}",
        runs()
    );
    let all: Vec<&str> = failed.iter().map(|s| s.as_str()).collect();
    assert!(failed.is_empty(), "{} of {} runs failed:\n{}", failed.len(), runs(), all.join("\n"));
}
