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

//! The used length on the legacy transport. The specification's legacy
//! notes have a driver ignore it there: many legacy devices set it wrong,
//! and bhyve's sets it to the status byte alone (`vq_relchain(..., 1)`) on
//! every chain. Held to the chain's writable bytes, every read on such a
//! host failed with E_IO. On legacy a length that claims the status byte
//! is enough and the status the device wrote is believed: it reads 0xFF
//! until the device writes it, and a read's buffer is emptied first, so a
//! device that wrote less than it said hands back zeros, never another
//! request's bytes. The modern transport keeps the exact rule (`used_tests`).

use super::model::{Rig, IRQ};
use super::script::answer_once;
use crate::io::{submit, BlkError};
use crate::queue::Direction;

fn answered(rig: &Rig, dir: Direction, len: u32, status: u8) -> Result<(), BlkError> {
    let mut q = rig.queue();
    answer_once(rig, move |d| d.used(1, 0, len, status));
    let r = submit(rig.transport(), &mut q, IRQ, dir, 0, 1);
    assert_eq!(q.stalled(), None, "an answered request stalled the queue");
    r
}

#[test]
fn a_legacy_read_answered_with_its_status_byte_alone_is_served_with_the_devices_data() {
    let rig = Rig::legacy();
    let mut q = rig.queue();
    answer_once(&rig, |d| {
        (0..512).for_each(|at| d.data.present8(at, 0x5A));
        d.used(1, 0, 1, 0);
    });
    let r = submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1);
    assert_eq!(r, Ok(()), "a read answered as bhyve answers it was refused");
    let mut out = [0u8; 512];
    assert!(q.take(&mut out) && out.iter().all(|&b| b == 0x5A), "not the device's bytes");
}

#[test]
fn any_legacy_length_that_claims_the_status_byte_is_taken() {
    let (read, write, flush) = (Direction::Read, Direction::Write, Direction::Flush);
    for (dir, len) in [(read, 1), (read, 513), (read, u32::MAX), (write, 1), (write, 513)]
        .into_iter()
        .chain([(flush, 1), (flush, 16 + 1)])
    {
        assert_eq!(answered(&Rig::legacy(), dir, len, 0), Ok(()), "{dir:?} answered {len}");
    }
}

#[test]
fn a_legacy_request_whose_status_was_never_written_or_claimed_fails() {
    for dir in [Direction::Read, Direction::Write, Direction::Flush] {
        let unset = answered(&Rig::legacy(), dir, 1, 0xFF);
        assert_eq!(unset, Err(BlkError::Io), "{dir:?}: the status still read 0xFF");
        let nothing = answered(&Rig::legacy(), dir, 0, 0);
        assert_eq!(nothing, Err(BlkError::Io), "{dir:?}: a length of nothing");
    }
}
