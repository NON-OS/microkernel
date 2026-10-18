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

//! How a request's end is answered: the device's status byte by name, and
//! each outcome by its wire status, a timeout as E_TIMEDOUT so a client can
//! tell a device that stopped answering from one that refused the sectors.

use super::model::{Rig, IRQ};
use super::script::answer_once;
use crate::io::{submit, BlkError};
use crate::protocol::{E_INVAL, E_IO, E_TIMEDOUT};
use crate::queue::Direction;
use crate::server::status::status_of;

fn read(rig: &Rig, q: &mut crate::queue::Queue) -> Result<(), BlkError> {
    submit(rig.transport(), q, IRQ, Direction::Read, 0, 1)
}

#[test]
fn the_device_status_is_answered_by_name() {
    let cases = [(0u8, Ok(())), (1, Err(BlkError::Io)), (2, Err(BlkError::Refused))];
    for (status, want) in cases.into_iter().chain([(0xFF, Err(BlkError::Io))]) {
        let rig = Rig::modern();
        let mut q = rig.queue();
        answer_once(&rig, move |d| d.used(1, 0, 513, status));
        assert_eq!(read(&rig, &mut q), want, "status {status:#x}");
    }
}

#[test]
fn each_outcome_has_its_own_wire_status() {
    assert_eq!(status_of(Ok(())), 0);
    assert_eq!(status_of(Err(BlkError::Io)), E_IO);
    assert_eq!(status_of(Err(BlkError::Refused)), E_INVAL);
    assert_eq!(status_of(Err(BlkError::Timeout)), E_TIMEDOUT);
    assert_eq!(E_TIMEDOUT, -110, "the errno the other block drivers send");
}
