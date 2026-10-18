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

//! The used element is the device's word, taken only when it names the
//! request in flight with the length that request has. Completion used to
//! be read from used.idx alone: an element naming another descriptor, or a
//! read the device answered OK without filling, was served, the latter
//! with the last request's bytes, which may have been another client's.

use nonos_libc::now;

use super::model::{Rig, IRQ};
use super::script::answer_once;
use crate::io::{submit, BlkError};
use crate::queue::{Direction, Fault};

fn read(rig: &Rig, q: &mut crate::queue::Queue) -> Result<(), BlkError> {
    submit(rig.transport(), q, IRQ, Direction::Read, 0, 1)
}

#[test]
fn a_completion_naming_another_descriptor_stalls_the_queue() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    answer_once(&rig, |d| d.used(1, 7, 513, 0));
    assert_eq!(read(&rig, &mut q), Err(BlkError::Io), "descriptor 7 was never posted");
    assert_eq!(q.stalled(), Some(Fault::Foreign));
}

#[test]
fn a_used_index_moved_past_the_request_is_caught_at_once() {
    for idx in [2u16, 9, u16::MAX] {
        let rig = Rig::modern();
        let mut q = rig.queue();
        answer_once(&rig, move |d| d.used(idx, 0, 513, 0));
        assert_eq!(read(&rig, &mut q), Err(BlkError::Io), "used.idx {idx}");
        assert_eq!((q.stalled(), now()), (Some(Fault::Foreign), 0), "waited on {idx}");
    }
}

// On the modern transport; the legacy rule is `legacy_len_tests`'.
#[test]
fn a_length_other_than_the_chains_fails_the_request_and_not_the_queue() {
    for (dir, len) in [(Direction::Read, 0), (Direction::Read, 1), (Direction::Read, 512)]
        .into_iter()
        .chain([(Direction::Read, 514), (Direction::Read, u32::MAX), (Direction::Write, 0)])
        .chain([(Direction::Write, 2), (Direction::Flush, 0), (Direction::Flush, 513)])
    {
        let rig = Rig::modern();
        let mut q = rig.queue();
        answer_once(&rig, move |d| d.used(1, 0, len, 0));
        let r = submit(rig.transport(), &mut q, IRQ, dir, 0, 1);
        assert_eq!((r, q.stalled()), (Err(BlkError::Io), None), "{dir:?} answered {len}");
    }
}
