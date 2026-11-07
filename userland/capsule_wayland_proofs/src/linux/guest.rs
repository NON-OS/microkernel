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

//! The guest the server serves, as far as the server looks at it: its
//! connection, its objects and scene, its descriptor table, and the memory
//! its pools are read from. On the host a guest's memory is this process's:
//! a pool is the client's descriptor mapped here (file.rs), and a read is
//! held to the spans mapped that way.

use std::cell::RefCell;

#[path = "../../../capsule_linux/src/linux/guest/memfd_spans.rs"]
pub mod memfd_spans;
pub use memfd_spans::MemfdSpan;
#[path = "../../../capsule_linux/src/linux/guest/mem_rows.rs"]
pub mod mem_rows;

/// What a descriptor slot holds.
#[derive(Debug)]
pub enum Held {
    Closed,
    /// A descriptor of this process, received from the client or made here.
    Host(i32),
    /// A memfd the server made and has not yet made real: its bytes, written
    /// when it is given to the client (session.rs).
    Memfd,
    /// One end of a pipe, standing for the guest's pipe `slot` (call.rs):
    /// one the server made (`ours`), or one the client passed.
    Pipe {
        slot: usize,
        write: bool,
        fd: i32,
        ours: bool,
    },
}

#[derive(Debug)]
pub struct Fd {
    pub held: Held,
    pub cloexec: bool,
    /// A memfd's length, and what is staged in it.
    pub size: u64,
    pub pending: Vec<u8>,
}

impl Fd {
    pub fn memfd() -> Fd {
        Fd { held: Held::Memfd, cloexec: false, size: 0, pending: Vec::new() }
    }

    pub fn host(fd: i32) -> Fd {
        Fd { held: Held::Host(fd), cloexec: false, size: 0, pending: Vec::new() }
    }

    pub fn pipe(slot: usize, write: bool, fd: i32, ours: bool) -> Fd {
        let held = Held::Pipe { slot, write, fd, ours };
        Fd { held, cloexec: false, size: 0, pending: Vec::new() }
    }

    pub fn is_open(&self) -> bool {
        !matches!(self.held, Held::Closed)
    }

    /// A second descriptor onto what this one names.
    pub fn clone_of(&self) -> Fd {
        // SAFETY: dup of a descriptor this process holds open.
        let dup = |fd: i32| unsafe { libc::dup(fd) };
        let held = match self.held {
            Held::Host(fd) => match dup(fd) {
                n if n >= 0 => Held::Host(n),
                _ => Held::Closed,
            },
            Held::Pipe { slot, write, fd, ours } => match dup(fd) {
                n if n >= 0 => Held::Pipe { slot, write, fd: n, ours },
                _ => Held::Closed,
            },
            Held::Memfd => Held::Memfd,
            Held::Closed => Held::Closed,
        };
        Fd { held, cloexec: self.cloexec, size: self.size, pending: self.pending.clone() }
    }

    /// Close what is held, leaving the slot free.
    pub fn close(&mut self) {
        if let Held::Host(fd) | Held::Pipe { fd, .. } = self.held {
            // SAFETY: a descriptor this slot owns, closed once.
            unsafe { libc::close(fd) };
        }
        self.held = Held::Closed;
    }
}

pub struct Guest {
    pub pid: u32,
    pub fds: Vec<Fd>,
    /// Pipe buffers, one a pipe (call.rs), and whether each one's write end
    /// has closed, as its reader has seen.
    pub pipes: Vec<Vec<u8>>,
    pub pipe_eof: Vec<bool>,
    pub display: crate::linux::unix::Conn,
    pub objects: crate::linux::wayland::Objects,
    pub scene: crate::linux::wayland::Scene,
    /// The pools' memfds as file.rs mapped them, which the server reads
    /// pixels by, as the capsule's are kept by mmap (memfd_spans.rs).
    pub memfd_spans: Vec<MemfdSpan>,
}

thread_local! {
    /// The spans of this process the client's pools are mapped at.
    static MAPPED: RefCell<Vec<(u64, u64)>> = const { RefCell::new(Vec::new()) };
}

/// A span mapped for a pool, which reads may now reach.
pub fn note_mapped(at: u64, len: u64) {
    MAPPED.with(|m| m.borrow_mut().push((at, len)));
}

/// The span mapped at `at` cut to its first `keep` bytes, none for zero, as
/// a guest's pages are once it unmaps them or exits: reads reaching past
/// the cut fail, as the kernel's do.
pub fn note_cut(at: u64, keep: u64) {
    MAPPED.with(|m| {
        let mut m = m.borrow_mut();
        m.iter_mut().filter(|(from, _)| *from == at).for_each(|(_, len)| *len = keep.min(*len));
        m.retain(|&(_, len)| len != 0);
    });
}

impl Guest {
    pub fn new(pid: u32) -> Guest {
        Guest {
            pid,
            fds: Vec::new(),
            pipes: Vec::new(),
            pipe_eof: Vec::new(),
            display: crate::linux::unix::Conn::new(),
            objects: crate::linux::wayland::Objects::new(),
            scene: crate::linux::wayland::Scene::new(),
            memfd_spans: Vec::new(),
        }
    }

    /// How many bytes from `addr` are mapped without a gap.
    pub fn mapped_from(&self, addr: u64) -> u64 {
        MAPPED.with(|m| {
            let m = m.borrow();
            m.iter()
                .filter(|(at, len)| *at <= addr && addr < at + len)
                .map(|(at, len)| at + len - addr)
                .max()
                .unwrap_or(0)
        })
    }

    /// Copy `out.len()` bytes from `addr`, only when all of them lie in one
    /// mapped span; a pool's buffer past its mapping reads nothing.
    pub fn read_into(_pid: u32, addr: u64, out: &mut [u8]) -> bool {
        let fits = MAPPED.with(|m| {
            m.borrow()
                .iter()
                .any(|(at, len)| *at <= addr && addr.saturating_add(out.len() as u64) <= at + len)
        });
        if fits {
            super::guest_reads::note(addr, out.len() as u64);
            // SAFETY: the span lies inside a live mapping made by file.rs.
            unsafe {
                core::ptr::copy_nonoverlapping(addr as *const u8, out.as_mut_ptr(), out.len())
            };
        }
        fits
    }

    /// `count` rows of `row` bytes, `stride` apart, copied from `addr` into
    /// `out` at the same offsets, in the kernel calls the capsule makes
    /// (mem_rows.rs), each counted as one; each row only when it lies in a
    /// mapped span. The bytes between the rows are not touched.
    pub fn read_rows_into(
        pid: u32,
        addr: u64,
        out: &mut [u8],
        (row, count, stride): (usize, usize, usize),
    ) -> bool {
        let row_at = |i: usize| (addr + (i * stride) as u64, i * stride);
        let Some(calls) = mem_rows::calls(row, count) else {
            return (0..count).all(|i| {
                let (from, at) = row_at(i);
                out.get_mut(at..at + row).is_some_and(|px| Guest::read_into(pid, from, px))
            });
        };
        for (first, n) in calls {
            let rows = first..first + n;
            if row > stride || rows.clone().any(|i| !mapped_whole(row_at(i).0, row as u64)) {
                return false;
            }
            if out.len() < (first + n - 1) * stride + row {
                return false;
            }
            super::guest_reads::note_rows(row_at(first).0, row as u64, n as u64, stride as u64);
            for i in rows {
                let (from, at) = row_at(i);
                // SAFETY: the row lies inside a live mapping made by file.rs,
                // and `out` was checked to reach its end.
                unsafe {
                    core::ptr::copy_nonoverlapping(from as *const u8, out[at..].as_mut_ptr(), row)
                };
            }
        }
        true
    }
}

/// Every span still mapped for a pool unmapped and forgotten, as a process's
/// mappings go when it exits. For a caller that runs many sessions on one
/// thread (the wayland_session fuzz target), once no Guest is left to read.
pub fn unmap_all() {
    MAPPED.with(|m| {
        for (at, len) in m.borrow_mut().drain(..) {
            // SAFETY: memfd_id mapped `len` bytes at `at` (note_cut only
            // shortens a span, and a shorter munmap leaves the rest mapped,
            // never frees what is not this mapping's), and with no Guest
            // left nothing reads through the span again.
            unsafe { libc::munmap(at as *mut libc::c_void, len as usize) };
        }
    });
}

/// Whether `len` bytes from `addr` lie in one mapped span.
fn mapped_whole(addr: u64, len: u64) -> bool {
    MAPPED.with(|m| {
        m.borrow().iter().any(|(at, n)| *at <= addr && addr.saturating_add(len) <= at + n)
    })
}

impl Drop for Guest {
    fn drop(&mut self) {
        self.fds.iter_mut().for_each(Fd::close);
    }
}
