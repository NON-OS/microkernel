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

//! One client's connection to the server, driven as the capsule's sendmsg,
//! recvmsg and serve loop drive it (unix/sendmsg.rs, unix/recvmsg.rs,
//! serve/loop_impl.rs): request bytes and descriptors in, then served;
//! input pumped, then events and descriptors out; held frame callbacks let
//! go when their period is up.

use crate::linux::guest::{Fd, Guest, Held};
use crate::linux::wayland::commit::pace;

/// The pid the server is told its guest has.
pub const PID: u32 = 7;

pub struct Session {
    pub guest: Guest,
}

impl Default for Session {
    fn default() -> Session {
        Session::new()
    }
}

impl Session {
    pub fn new() -> Session {
        crate::linux::wayland::commit::launch_clock::connected();
        Session { guest: Guest::new(PID) }
    }

    /// The client wrote `bytes`, passing `fds`, which the session now owns.
    /// How many bytes were taken, as a stream socket's sendmsg says.
    pub fn send(&mut self, bytes: &[u8], fds: &[i32]) -> usize {
        for &fd in fds {
            let entry = received(&mut self.guest, fd);
            let slot = crate::linux::file::install(&mut self.guest, entry);
            self.guest.display.fds.extend(slot.map(|s| s as u32));
        }
        let take = bytes.len().min(self.guest.display.room());
        self.guest.display.to_server.extend_from_slice(&bytes[..take]);
        crate::linux::wayland::serve(&mut self.guest);
        self.flush_pipes();
        self.collect();
        take
    }

    /// What the client reads: input taken in first, then up to `want` bytes
    /// of events and every descriptor owed, whose ownership passes to the
    /// caller. Requests that waited for the client to read go on after.
    pub fn recv(&mut self, want: usize) -> (Vec<u8>, Vec<i32>) {
        self.read_pipes();
        crate::linux::wayland::pump(&mut self.guest);
        let given = core::mem::take(&mut self.guest.display.give);
        let fds = given.into_iter().filter_map(|slot| self.hand_over(slot)).collect();
        let bytes = self.guest.display.drain(want);
        crate::linux::wayland::serve(&mut self.guest);
        self.flush_pipes();
        self.collect();
        (bytes, fds)
    }

    /// Frame callbacks whose period has passed answered; when the next is due.
    pub fn tick(&mut self) -> Option<u64> {
        let now = crate::linux::call::family_ms();
        pace::settle(&mut self.guest, now);
        pace::next_ms(&self.guest, now)
    }

    /// Whether events wait for the client.
    pub fn pending(&self) -> bool {
        !self.guest.display.to_client.is_empty() || !self.guest.display.give.is_empty()
    }

    /// The client closed its end: the window goes, as the capsule's close does.
    pub fn disconnect(&mut self) {
        crate::linux::wayland::disconnect(&mut self.guest);
    }

    /// The descriptor in `slot` as one of this process, its slot left free.
    fn hand_over(&mut self, slot: u32) -> Option<i32> {
        let f = self.guest.fds.get_mut(slot as usize)?;
        let fd = match f.held {
            Held::Host(fd) | Held::Pipe { fd, .. } => fd,
            Held::Memfd => memfd(&f.pending, f.size)?,
            Held::Closed => return None,
        };
        f.held = Held::Closed;
        Some(fd)
    }

    /// A received descriptor no request is waiting to take and no pool is
    /// read through is closed: the client keeps its own. A pipe end it passed
    /// is closed once what was put in it is written, so its reader sees the
    /// end, as in the capsule, where the client has closed its own.
    fn collect(&mut self) {
        let g = &mut self.guest;
        for (i, f) in g.fds.iter_mut().enumerate() {
            let i = i as u32;
            let kept = g.display.fds.contains(&i)
                || g.display.give.contains(&i)
                || g.scene.pools.iter().any(|p| p.fd == i);
            let theirs = matches!(f.held, Held::Host(_) | Held::Pipe { ours: false, .. });
            if !kept && theirs {
                f.close();
            }
        }
    }

    /// What the server put in a pipe's buffer, written to its write end.
    fn flush_pipes(&mut self) {
        let g = &mut self.guest;
        for f in &g.fds {
            let Held::Pipe { slot, write: true, fd, .. } = f.held else { continue };
            let Some(buf) = g.pipes.get_mut(slot).filter(|b| !b.is_empty()) else { continue };
            // SAFETY: a pipe end this slot holds open; a short write keeps the rest.
            let n = unsafe { libc::write(fd, buf.as_ptr().cast(), buf.len()) };
            buf.drain(..n.max(0) as usize);
        }
    }

    /// What a client wrote to a pipe the server reads, into its buffer; the
    /// end, once its writer has closed.
    fn read_pipes(&mut self) {
        let g = &mut self.guest;
        let mut chunk = [0u8; 4096];
        for f in &g.fds {
            let Held::Pipe { slot, write: false, fd, .. } = f.held else { continue };
            loop {
                // SAFETY: a read end this slot holds open, set not to block.
                let n = unsafe { libc::read(fd, chunk.as_mut_ptr().cast(), chunk.len()) };
                match n {
                    0 => g.pipe_eof[slot] = true,
                    n if n > 0 => g.pipes[slot].extend_from_slice(&chunk[..n as usize]),
                    _ => {}
                }
                if n <= 0 {
                    break;
                }
            }
        }
    }
}

/// A descriptor the client passed: a pipe's end as the guest pipe it is in
/// the capsule, any other as itself.
fn received(guest: &mut Guest, fd: i32) -> Fd {
    // SAFETY: fstat and fcntl of a descriptor just received.
    let (fifo, write) = unsafe {
        let mut st: libc::stat = core::mem::zeroed();
        let fifo = libc::fstat(fd, &mut st) == 0 && st.st_mode & libc::S_IFMT == libc::S_IFIFO;
        (fifo, libc::fcntl(fd, libc::F_GETFL) & libc::O_ACCMODE == libc::O_WRONLY)
    };
    match fifo {
        true => Fd::pipe(crate::linux::call::new_slot(guest), write, fd, false),
        false => Fd::host(fd),
    }
}

/// A memfd of `size` bytes holding `bytes`.
fn memfd(bytes: &[u8], size: u64) -> Option<i32> {
    // SAFETY: a new descriptor, written and sized before it is handed over.
    unsafe {
        let fd = libc::memfd_create(c"nonos-wayland".as_ptr(), libc::MFD_CLOEXEC);
        if fd < 0 {
            return None;
        }
        let len = size.max(bytes.len() as u64) as libc::off_t;
        let wrote = libc::write(fd, bytes.as_ptr().cast(), bytes.len());
        if wrote != bytes.len() as isize || libc::ftruncate(fd, len) != 0 {
            libc::close(fd);
            return None;
        }
        Some(fd)
    }
}
