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

//! `read`, `write` and `close`, routed by what the descriptor is.

use crate::linux::abi::errno;
use crate::linux::file;
use crate::linux::net;

use super::console::console;

/// Port 53 on the loopback address, in network order. What a program
/// that wrote to its nameserver without naming one was talking to.
const LOOPBACK_53: [u8; 6] = [0, 53, 127, 0, 0, 1];

use super::io_socket::{socket_read, socket_write};
use crate::linux::guest::{Guest, Kind};

pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    match guest.fds.get(fd as usize).map(|f| &f.kind) {
        Some(Kind::Stdout) => console(guest, buf, len, 1),
        Some(Kind::Stderr) => console(guest, buf, len, 2),
        Some(Kind::File) => file::write(guest, fd, buf, len),
        Some(Kind::Socket) => socket_write(guest, fd, buf, len),
        Some(Kind::Unix) => crate::linux::unix::send(guest, fd, buf, len),
        Some(Kind::Pipe) => super::pipe_io::write(guest, fd, buf, len),
        Some(Kind::Event) => file::event_write(guest, fd, buf, len),
        Some(Kind::Device) => file::dev_write(guest, fd, buf, len),
        Some(Kind::Resolver) => net::dns::query(guest, fd, buf, len, LOOPBACK_53),
        Some(Kind::Dir) => errno::fail(errno::EISDIR),
        _ => errno::fail(errno::EBADF),
    }
}
pub fn read(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    match guest.fds.get(fd as usize).map(|f| &f.kind) {
        /*
         * What the terminal typed, or end of file on no terminal.
         */
        Some(Kind::Stdin) => crate::linux::console::read(guest, buf, len),
        Some(Kind::File) => file::read(guest, fd, buf, len),
        Some(Kind::Timer) => file::timerfd_read(guest, fd, buf, len),
        Some(Kind::Socket) => socket_read(guest, fd, buf, len),
        Some(Kind::Unix) => crate::linux::unix::recv(guest, fd, buf, len),
        Some(Kind::Pipe) => super::pipe_read::read(guest, fd, buf, len),
        Some(Kind::Event) => file::event_read(guest, fd, buf, len),
        Some(Kind::Device) => file::dev_read(guest, fd, buf, len),
        Some(Kind::Signal) => super::signalfd_now(guest, fd, buf, len),
        Some(Kind::Resolver) => net::dns::answer_out(guest, fd, buf, len).0,
        Some(Kind::Dir) => errno::fail(errno::EISDIR),
        _ => errno::fail(errno::EBADF),
    }
}

pub fn close(guest: &mut Guest, fd: u64) -> u64 {
    net::close(guest, fd);
    file::close(guest, fd)
}
