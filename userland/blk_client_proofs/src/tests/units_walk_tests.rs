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

//! The walk over one service's units: every unit asked, past a gap, the
//! gaps left out, a driver that does not answer asked once; and two disks
//! on one service told apart by the word that confirms the erase.

use crate::error::BlkError;
use crate::units::each_unit;
use crate::word::bus_word;

#[test]
fn every_unit_is_asked_and_the_gaps_left_out() {
    let mut asked = Vec::new();
    let rows = each_unit(4, |u| {
        asked.push(u);
        match u {
            0 | 2 => Ok(None),
            1 => Ok(Some("primary slave")),
            _ => Err(BlkError::Status(-5)),
        }
    });
    assert_eq!(asked, [0, 1, 2, 3], "past a gap too: IDE units are channel and drive");
    assert_eq!(rows, [(1, Ok("primary slave")), (3, Err(BlkError::Status(-5)))]);
}

#[test]
fn a_driver_that_does_not_answer_is_asked_once() {
    let mut asked = 0;
    let rows = each_unit::<u8>(16, |_| {
        asked += 1;
        Err(BlkError::Transport(-110))
    });
    assert_eq!((asked, rows), (1, vec![(0, Err(BlkError::Transport(-110)))]));
}

#[test]
fn two_disks_on_one_service_have_two_words() {
    assert_eq!(bus_word("ide", 0, 0), "ide");
    assert_eq!(bus_word("ide", 0, 1), "ide-1");
    assert_eq!(bus_word("scsi", 1, 0), "scsi1");
    assert_eq!(bus_word("scsi", 1, 3), "scsi1-3");
}
