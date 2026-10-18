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

//! One step of the part: serve the next head if the driver published one.

use std::sync::atomic::{fence, Ordering};
use std::sync::Mutex;

use nonos_devmodel::FakeBar;

use super::ring::{put, u16_at};
use super::{answers, chain, Answer, Seen};
use crate::constants::{VQ_AVAIL_OFFSET, VQ_USED_OFFSET};
use crate::tests::model::QUEUE_SIZE;

/// The available and used indices, as a step begins.
pub fn indices(bar: &FakeBar) -> (u16, u16) {
    (u16_at(bar, VQ_AVAIL_OFFSET + 2), u16_at(bar, VQ_USED_OFFSET + 2))
}

/// Serve the head after the `done` the part answered, if the indices `at`
/// say the driver published one, noting the device `status` it was taken
/// under: whether the part answered it. Nothing is written unless the ring
/// still reads `at`: the driver may have reset the device and restarted the
/// queue meanwhile, and what was read then is not the new queue's command.
pub fn step(
    bar: &FakeBar,
    at: (u16, u16),
    done: u16,
    log: &Mutex<Vec<Seen>>,
    answer: Answer,
    status: u8,
) -> bool {
    if at.0 == done {
        return false;
    }
    fence(Ordering::Acquire);
    let slot = (done % QUEUE_SIZE) as usize;
    let Some(chain) = chain::read(bar, slot).filter(|c| !answers::left(&c.request, answer)) else {
        return false;
    };
    let response = answers::build(&chain.request, chain.room, answer);
    if indices(bar) != at {
        return false;
    }
    put(bar, chain.answer_at, &response);
    let (request_flags, response_flags) = chain.flags;
    let seen = Seen { request: chain.request, request_flags, response_flags, status };
    log.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).push(seen);
    let id = match answer {
        Answer::WrongDescriptor => (chain.head.wrapping_add(1) % QUEUE_SIZE) as u32,
        Answer::AliasedDescriptor => chain.head as u32 | 0x1_0000,
        _ => chain.head as u32,
    };
    let entry = VQ_USED_OFFSET + 4 + slot * 8;
    put(bar, entry, &id.to_le_bytes());
    put(bar, entry + 4, &(response.len() as u32).to_le_bytes());
    fence(Ordering::Release);
    let moved = if answer == Answer::SkippedIndex { 2 } else { 1 };
    put(bar, VQ_USED_OFFSET + 2, &done.wrapping_add(moved).to_le_bytes());
    true
}
