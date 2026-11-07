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

//! One client on its socket: what it sends taken with its descriptors, what
//! it is owed written with its descriptors, as libwayland's sendmsg and
//! recvmsg carry them.

use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;

use crate::record::{self, Recorder};
use crate::session::Session;

/// The most descriptors one message carries, libwayland's own bound.
const MOST_FDS: usize = 28;

pub struct Client {
    pub sock: UnixStream,
    pub session: Session,
    /// Request bytes the server had no room for yet, and event bytes and
    /// descriptors the socket had no room for yet.
    held: Vec<u8>,
    held_fds: Vec<i32>,
    out: Vec<u8>,
    out_fds: Vec<i32>,
    /// Where this session is recorded, when NONOS_WAYLAND_RECORD names a file.
    pub rec: Option<Recorder>,
}

impl Client {
    pub fn new(sock: UnixStream) -> Client {
        sock.set_nonblocking(true).ok();
        Client {
            sock,
            session: Session::new(),
            held: Vec::new(),
            held_fds: Vec::new(),
            out: Vec::new(),
            out_fds: Vec::new(),
            rec: recorder(),
        }
    }

    pub fn poll(&self) -> libc::pollfd {
        let mut events = libc::POLLIN;
        if !self.out.is_empty() {
            events |= libc::POLLOUT;
        }
        libc::pollfd { fd: self.sock.as_raw_fd(), events, revents: 0 }
    }

    pub fn tick(&mut self) -> Option<u64> {
        self.session.tick()
    }

    /// Read what came, serve it, write what is owed. False once the client
    /// has gone.
    pub fn turn(&mut self) -> bool {
        loop {
            match recv(self.sock.as_raw_fd()) {
                Recv::Got(bytes, fds) => {
                    let lens: Vec<u64> = fds.iter().map(|&fd| record::fd_len(fd)).collect();
                    self.rec.iter_mut().for_each(|r| r.put(&record::read_chunk(&bytes, &lens)));
                    self.held.extend_from_slice(&bytes);
                    self.held_fds.extend(fds);
                    self.feed();
                }
                Recv::Empty => break,
                Recv::Gone => {
                    eprintln!("[host] client disconnected");
                    self.session.disconnect();
                    return false;
                }
            }
        }
        self.feed();
        self.flush();
        true
    }

    /// Hand the server what it has room for, reading events out between.
    fn feed(&mut self) {
        while !self.held.is_empty() {
            let fds = core::mem::take(&mut self.held_fds);
            let n = self.session.send(&self.held, &fds);
            self.held.drain(..n);
            self.flush();
            if n == 0 {
                return;
            }
        }
    }

    fn flush(&mut self) {
        let (bytes, fds) = self.session.recv(usize::MAX);
        self.out.extend_from_slice(&bytes);
        self.out_fds.extend(fds);
        while !self.out.is_empty() {
            let take = self.out_fds.len().min(MOST_FDS);
            let n = send(self.sock.as_raw_fd(), &self.out, &self.out_fds[..take]);
            if n <= 0 {
                return;
            }
            // SAFETY: the descriptors went with the message; ours are closed.
            self.out_fds.drain(..take).for_each(|fd| unsafe {
                libc::close(fd);
            });
            self.out.drain(..n as usize);
        }
    }
}

/// A record file per client: the path NONOS_WAYLAND_RECORD names, numbered.
fn recorder() -> Option<Recorder> {
    use core::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    let path = std::env::var("NONOS_WAYLAND_RECORD").ok()?;
    Recorder::create(&format!("{path}.{}", N.fetch_add(1, Ordering::Relaxed)))
}

enum Recv {
    Got(Vec<u8>, Vec<i32>),
    Empty,
    Gone,
}

fn recv(sock: i32) -> Recv {
    let mut buf = vec![0u8; 1 << 16];
    let mut cbuf = [0u64; 64];
    let mut iov = libc::iovec { iov_base: buf.as_mut_ptr().cast(), iov_len: buf.len() };
    // SAFETY: msghdr over buffers this frame owns; the control block is
    // walked with the libc macros within its returned length.
    unsafe {
        let mut msg: libc::msghdr = core::mem::zeroed();
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1;
        msg.msg_control = cbuf.as_mut_ptr().cast();
        msg.msg_controllen = core::mem::size_of_val(&cbuf) as _;
        let n = libc::recvmsg(sock, &mut msg, libc::MSG_CMSG_CLOEXEC | libc::MSG_DONTWAIT);
        if n == 0 {
            return Recv::Gone;
        }
        if n < 0 {
            let e = *libc::__errno_location();
            return if e == libc::EAGAIN || e == libc::EINTR { Recv::Empty } else { Recv::Gone };
        }
        let mut fds = Vec::new();
        let mut c = libc::CMSG_FIRSTHDR(&msg);
        while !c.is_null() {
            if (*c).cmsg_level == libc::SOL_SOCKET && (*c).cmsg_type == libc::SCM_RIGHTS {
                let data = libc::CMSG_DATA(c) as *const i32;
                let count = ((*c).cmsg_len as usize - libc::CMSG_LEN(0) as usize) / 4;
                fds.extend((0..count).map(|i| data.add(i).read_unaligned()));
            }
            c = libc::CMSG_NXTHDR(&msg, c);
        }
        buf.truncate(n as usize);
        Recv::Got(buf, fds)
    }
}

fn send(sock: i32, bytes: &[u8], fds: &[i32]) -> isize {
    let mut cbuf = [0u64; 32];
    let mut iov = libc::iovec { iov_base: bytes.as_ptr() as *mut _, iov_len: bytes.len() };
    // SAFETY: msghdr over buffers this frame owns; the one control message
    // is sized by CMSG_SPACE for `fds`, within `cbuf`.
    unsafe {
        let mut msg: libc::msghdr = core::mem::zeroed();
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1;
        if !fds.is_empty() {
            let len = fds.len() * 4;
            msg.msg_control = cbuf.as_mut_ptr().cast();
            msg.msg_controllen = libc::CMSG_SPACE(len as u32) as _;
            let c = libc::CMSG_FIRSTHDR(&msg);
            (*c).cmsg_level = libc::SOL_SOCKET;
            (*c).cmsg_type = libc::SCM_RIGHTS;
            (*c).cmsg_len = libc::CMSG_LEN(len as u32) as _;
            core::ptr::copy_nonoverlapping(fds.as_ptr(), libc::CMSG_DATA(c) as *mut i32, fds.len());
        }
        libc::sendmsg(sock, &msg, libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL)
    }
}
