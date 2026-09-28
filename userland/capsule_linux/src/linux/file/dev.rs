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

//! The character devices every Linux program may assume: /dev/null, /dev/zero,
//! /dev/full, /dev/random and /dev/urandom. They are not files in the store;
//! each is a descriptor this capsule answers itself, with the major and minor
//! numbers Linux gives it, so fstat and stat say character device as Linux's.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::flags::writes;
use super::slot::install;

/// Path, major, minor. The handle of an open device is its index here.
const DEVICES: [(&[u8], u64, u64); 5] = [
    (b"/dev/null", 1, 3),
    (b"/dev/zero", 1, 5),
    (b"/dev/full", 1, 7),
    (b"/dev/random", 1, 8),
    (b"/dev/urandom", 1, 9),
];
pub const NULL: u32 = 0;
pub const ZERO: u32 = 1;
pub const FULL: u32 = 2;

/// The device a guest-visible path names, if it names one.
pub fn device_of(full: &[u8]) -> Option<u32> {
    DEVICES.iter().position(|(p, _, _)| *p == full).map(|i| i as u32)
}

/// Open the device `full` names; the caller has checked that it names one.
pub fn open_path(guest: &mut Guest, full: &[u8], flags: u64) -> u64 {
    let Some(dev) = device_of(full) else {
        return errno::fail(errno::ENOENT);
    };
    let mut fd = Fd::empty(Kind::Device);
    fd.handle = dev;
    fd.path = full.to_vec();
    fd.writable = writes(flags);
    match install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

/// The major and minor numbers of an open device.
pub fn numbers(dev: u32) -> Option<(u64, u64)> {
    DEVICES.get(dev as usize).map(|&(_, major, minor)| (major, minor))
}

/// Whether epoll may watch it: null, zero and full have no poll on Linux and
/// are refused with EPERM; the random devices can be waited on.
pub fn polls(dev: u32) -> bool {
    dev > FULL
}
