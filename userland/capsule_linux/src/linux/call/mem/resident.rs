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
//! `mincore`: which pages of a span are resident.
//!
//! A backed span is resident page for page, since the personality maps every
//! page of it when it is committed and the kernel pages nothing out, whatever
//! its protection now. A reservation has no page at all. So the region list
//! answers exactly, one byte per page, 1 for resident.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{page_up, Guest, MAX_SPAN, PAGE, USER_MAX};

pub fn mincore(guest: &Guest, addr: u64, len: u64, vec: u64) -> u64 {
    if addr % PAGE != 0 {
        return errno::fail(errno::EINVAL);
    }
    let Some(end) = addr.checked_add(len).filter(|e| *e <= USER_MAX) else {
        return errno::fail(errno::ENOMEM);
    };
    let end = page_up(end);
    let mut at = addr;
    let mut out: Vec<u8> = Vec::new();
    let mut written = 0u64;
    while at < end {
        let Some(r) = guest.regions.iter().find(|r| r.at <= at && at < r.at + r.len) else {
            return errno::fail(errno::ENOMEM);
        };
        out.push(r.backed as u8);
        at += PAGE;
        if out.len() as u64 == MAX_SPAN || at >= end {
            if guest.write(vec + written, &out) < out.len() as i64 {
                return errno::fail(errno::EFAULT);
            }
            written += out.len() as u64;
            out.clear();
        }
    }
    errno::ok(0)
}
