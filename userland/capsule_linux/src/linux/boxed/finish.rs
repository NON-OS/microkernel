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

//! The report a boxed run ends with, sent to the terminal that started it
//! on the frames only it reads (frames.rs), and waited for while its inbox
//! is full: a guest that fills the inbox as it exits cannot drop the report.

use nonos_libc::{mk_idle_ms, mk_private_write, Deadline};

use super::report::End;
use super::state::BOX;
use crate::linux::abi::errno::EBUSY;

const _: () = assert!(super::frames::TAG == nonos_libc::TAG_BOX);

const WAIT_MS: u64 = 2_000;
const GAP_MS: u64 = 5;

pub fn finish(exit: i32) {
    end(End { exit, not_started: None, reached: &[] });
}

/// The run stopped before its guest started, and why.
pub fn not_started(why: &str, exit: i32) {
    end(End { exit, not_started: Some(why), reached: &[] });
}

fn end(e: End<'_>) {
    let Some(a) = BOX.0.borrow_mut().take() else { return };
    let e = End { reached: a.connected.all(), ..e };
    let mut text = super::report::report(&a.policy, &a.hash, &a.tally, a.granted, &e);
    let learned = super::learn_report::learned(&a.policy, &a.learned, &a.tally);
    text.push_str(&learned.unwrap_or_default());
    text.push_str(&super::net_report::net_line(&a.policy, e.reached).unwrap_or_default());
    text.push_str(&super::line::box_line(&a.policy, &a.hash, &a.tally, &e));
    match a.key {
        Some(key) => send(&super::frames::frames(&key, text.as_bytes())),
        None => crate::linux::say::say(text.as_bytes()),
    }
}

fn send(frames: &[alloc::vec::Vec<u8>]) {
    let until = Deadline::after_ms(WAIT_MS);
    for f in frames {
        while mk_private_write(f) == -EBUSY && !until.expired() {
            let _ = mk_idle_ms(GAP_MS);
        }
    }
}
