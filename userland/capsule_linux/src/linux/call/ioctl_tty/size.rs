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

//! The terminal requests about its window size, its queues and its flow,
//! and the answer written back to the guest.

use crate::linux::abi::errno;
use crate::linux::console;
use crate::linux::guest::siginfo::{SigInfo, SI_KERNEL};
use crate::linux::guest::Guest;

use super::super::ioctl_req::{flow_ok, flush_input, winsize, Req};

/// SIGWINCH, raised when TIOCSWINSZ changes the size.
const SIGWINCH: u8 = 28;

/// `bytes` written at `arg`: zero, or EFAULT.
pub(super) fn put(guest: &Guest, arg: u64, bytes: &[u8]) -> u64 {
    match guest.write(arg, bytes) == bytes.len() as i64 {
        true => errno::ok(0),
        false => errno::fail(errno::EFAULT),
    }
}

/// The answer to `req`, one of the requests here; ENOTTY for any other.
pub(super) fn size(guest: &mut Guest, req: Req, arg: u64) -> u64 {
    match req {
        Req::GetWinsize => {
            let (rows, cols) = console::size();
            let mut ws = [0u8; 8];
            ws[..2].copy_from_slice(&rows.to_le_bytes());
            ws[2..4].copy_from_slice(&cols.to_le_bytes());
            put(guest, arg, &ws)
        }
        Req::SetWinsize => match guest.read(arg, 8).as_deref().and_then(winsize) {
            Some((rows, cols)) => {
                if console::set_size(rows, cols) {
                    let _ = guest.raise_and_wake(0, SigInfo::from(SIGWINCH, SI_KERNEL, 0));
                }
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        Req::OutQueued => put(guest, arg, &0i32.to_le_bytes()),
        Req::Flush => match flush_input(arg) {
            Ok(input) => {
                if input {
                    console::flush();
                }
                errno::ok(0)
            }
            Err(e) => errno::fail(e),
        },
        Req::Drain => errno::ok(0),
        Req::FlowControl => match flow_ok(arg) {
            Ok(()) => errno::ok(0),
            Err(e) => errno::fail(e),
        },
        _ => errno::fail(errno::ENOTTY),
    }
}
