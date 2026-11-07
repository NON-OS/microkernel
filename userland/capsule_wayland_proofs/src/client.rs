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

//! A client written by hand: requests built word by word, events read back
//! off the wire, against a fresh server session. Each test names the objects
//! it makes; the globals are bound by their name in the server's registry.


use crate::session::Session;

/// One argument of a request.
pub enum A<'a> {
    U(u32),
    I(i32),
    S(&'a str),
}

/// One event: the object, the opcode, and its argument words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ev {
    pub object: u32,
    pub opcode: u16,
    pub body: Vec<u8>,
}

impl Ev {
    pub fn word(&self, i: usize) -> u32 {
        u32::from_le_bytes(self.body[i * 4..i * 4 + 4].try_into().expect("a whole word"))
    }

    /// The array starting at word `i`, as words.
    pub fn array(&self, i: usize) -> Vec<u32> {
        let len = self.word(i) as usize / 4;
        (0..len).map(|n| self.word(i + 1 + n)).collect()
    }
}

pub struct Client {
    pub s: Session,
}

impl Default for Client {
    fn default() -> Client {
        Client::new()
    }
}

impl Client {
    pub fn new() -> Client {
        Client { s: Session::new() }
    }

    /// Send one request, with `fds` passed alongside.
    pub fn req_fds(&mut self, object: u32, opcode: u16, args: &[A], fds: &[i32]) {
        let mut body = Vec::new();
        for a in args {
            match a {
                A::U(v) => body.extend_from_slice(&v.to_le_bytes()),
                A::I(v) => body.extend_from_slice(&v.to_le_bytes()),
                A::S(t) => {
                    body.extend_from_slice(&(t.len() as u32 + 1).to_le_bytes());
                    body.extend_from_slice(t.as_bytes());
                    body.push(0);
                    while body.len() % 4 != 0 {
                        body.push(0);
                    }
                }
            }
        }
        let mut msg = object.to_le_bytes().to_vec();
        msg.extend_from_slice(&opcode.to_le_bytes());
        msg.extend_from_slice(&((body.len() + 8) as u16).to_le_bytes());
        msg.extend_from_slice(&body);
        assert_eq!(self.s.send(&msg, fds), msg.len(), "the server took the whole request");
    }

    pub fn req(&mut self, object: u32, opcode: u16, args: &[A]) {
        self.req_fds(object, opcode, args, &[]);
    }

    /// Every event waiting, read off the wire; descriptors are closed.
    pub fn events(&mut self) -> Vec<Ev> {
        let (bytes, fds) = self.s.recv(usize::MAX);
        // SAFETY: descriptors handed over by the session, closed once.
        fds.into_iter().for_each(|fd| unsafe {
            libc::close(fd);
        });
        let mut out = Vec::new();
        let mut at = 0;
        while at + 8 <= bytes.len() {
            let object = u32::from_le_bytes(bytes[at..at + 4].try_into().expect("a word"));
            let opcode = u16::from_le_bytes(bytes[at + 4..at + 6].try_into().expect("a half"));
            let size =
                u16::from_le_bytes(bytes[at + 6..at + 8].try_into().expect("a half")) as usize;
            out.push(Ev { object, opcode, body: bytes[at + 8..at + size].to_vec() });
            at += size;
        }
        assert_eq!(at, bytes.len(), "every byte is a whole event");
        out
    }

    /// The registry at `registry`, and each global bound at `version` to
    /// the id given, by its interface name.
    pub fn bind(&mut self, registry: u32, globals: &[(&str, u32, u32)]) {
        self.req(1, 1, &[A::U(registry)]);
        let offered = self.events();
        for &(iface, version, id) in globals {
            let name = offered
                .iter()
                .find(|e| e.object == registry && iface_of(e) == iface)
                .map(|e| e.word(0))
                .unwrap_or_else(|| panic!("{iface} is offered"));
            self.req(registry, 0, &[A::U(name), A::S(iface), A::U(version), A::U(id)]);
        }
    }

    /// A shm buffer `id` of `w` by `h`, in a pool `pool` of its own, over a
    /// memfd mapped by the server.
    pub fn buffer(&mut self, shm: u32, pool: u32, id: u32, (w, h): (i32, i32)) {
        self.painted(shm, pool, id, (w, h), 0);
    }

    /// The same, every pixel `argb`.
    pub fn painted(&mut self, shm: u32, pool: u32, id: u32, (w, h): (i32, i32), argb: u32) {
        let size = w * h * 4;
        let px: Vec<u8> = (0..w * h).flat_map(|_| argb.to_le_bytes()).collect();
        // SAFETY: a new memfd, written and sized, handed to the session with
        // the request.
        let fd = unsafe {
            let fd = libc::memfd_create(c"client".as_ptr(), libc::MFD_CLOEXEC);
            assert!(fd >= 0 && libc::ftruncate(fd, size as libc::off_t) == 0);
            assert_eq!(libc::write(fd, px.as_ptr().cast(), px.len()), px.len() as isize);
            fd
        };
        self.req_fds(shm, 0, &[A::U(pool), A::I(size)], &[fd]);
        self.req(pool, 0, &[A::U(id), A::I(0), A::I(w), A::I(h), A::I(w * 4), A::U(0)]);
    }
}

/// A wl_registry.global's interface name.
fn iface_of(e: &Ev) -> String {
    let len = u32::from_le_bytes(e.body[4..8].try_into().expect("a word")) as usize;
    String::from_utf8_lossy(&e.body[8..8 + len - 1]).into_owned()
}

/// The ids every test uses for the same things.
pub const REGISTRY: u32 = 2;
pub const COMPOSITOR: u32 = 3;
pub const WM_BASE: u32 = 4;
pub const SEAT: u32 = 5;
pub const SHM: u32 = 6;
pub const SURFACE: u32 = 10;
pub const XDG_SURFACE: u32 = 11;
pub const TOPLEVEL: u32 = 12;
pub const KEYBOARD: u32 = 13;
pub const POOL: u32 = 20;
pub const BUFFER: u32 = 21;

/// A client with a compositor, xdg_wm_base at `wm`, a seat with a keyboard,
/// and shm; a surface made a toplevel, not yet committed.
pub fn toplevel(wm: u32) -> Client {
    let mut c = Client::new();
    c.bind(
        REGISTRY,
        &[
            ("wl_compositor", 6, COMPOSITOR),
            ("xdg_wm_base", wm, WM_BASE),
            ("wl_seat", 5, SEAT),
            ("wl_shm", 1, SHM),
        ],
    );
    c.req(SEAT, 1, &[A::U(KEYBOARD)]);
    c.req(COMPOSITOR, 0, &[A::U(SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(XDG_SURFACE), A::U(SURFACE)]);
    c.req(XDG_SURFACE, 1, &[A::U(TOPLEVEL)]);
    c.events();
    c
}

/// The events sent to `object`.
pub fn to(evs: &[Ev], object: u32) -> Vec<&Ev> {
    evs.iter().filter(|e| e.object == object).collect()
}
