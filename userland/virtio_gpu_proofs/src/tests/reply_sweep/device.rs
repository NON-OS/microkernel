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

//! A device answering the next command with given bytes and a used length
//! of its choosing, on the driver's own thread.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{off_yield, on_yield};

use crate::constants::{VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_USED_OFFSET};
use crate::device::ControlQueue;
use crate::tests::harness::{harness, Harness};
use crate::tests::host::words::at;
use crate::tests::part::ring::{desc, put, u16_at};

/// The device's bytes (cut to the buffer offered) and claimed length.
#[derive(Clone, Debug)]
pub struct Reply {
    pub bytes: Vec<u8>,
    pub claimed: Option<u32>,
}

/// A queue whose next command is answered with `reply`, noting the room.
fn answering(reply: Reply, room: Rc<RefCell<u32>>) -> Harness {
    let h = harness();
    let region = Arc::clone(&h.region);
    let mut served = false;
    on_yield(move || {
        if !served && u16_at(&region, VQ_AVAIL_OFFSET + 2) != 0 {
            served = true;
            serve(&region, &reply, &room);
        }
    });
    h
}

fn serve(bar: &FakeBar, reply: &Reply, room: &RefCell<u32>) {
    let head = u16_at(bar, VQ_AVAIL_OFFSET + 4);
    let second = desc(bar, desc(bar, head, VQ_DESC_OFFSET).next, VQ_DESC_OFFSET);
    *room.borrow_mut() = second.len;
    let n = reply.bytes.len().min(second.len as usize);
    put(bar, at(bar, second.addr, second.len), &reply.bytes[..n]);
    let claimed = reply.claimed.unwrap_or(n as u32);
    put(bar, VQ_USED_OFFSET + 4, &(head as u32).to_le_bytes());
    put(bar, VQ_USED_OFFSET + 8, &claimed.to_le_bytes());
    put(bar, VQ_USED_OFFSET + 2, &1u16.to_le_bytes());
}

/// Run `ask` on a queue whose device answers with `reply`: its result,
/// the room the driver offered, and whether the queue is still in service.
pub fn case<T>(reply: Reply, ask: impl FnOnce(&ControlQueue) -> T) -> (T, u32, bool) {
    let room = Rc::new(RefCell::new(0));
    let h = answering(reply, Rc::clone(&room));
    let out = ask(&h.queue);
    off_yield();
    let offered = *room.borrow();
    (out, offered, !h.queue.is_reset())
}
