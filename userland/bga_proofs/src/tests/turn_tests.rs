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

//! The serve loop's rule for sender 0 (`turn`): the kernel's DGON for this
//! driver's device ends it, one for another device leaves it running, and
//! any other frame from sender 0 is dropped, which the loop does before it
//! could answer. A DGON's bytes from a process are served as a request.

use crate::dgon::{dgon_encode, Dgon, DGON_LEN, WHY_GONE};
use crate::turn::{turn, Turn};

const OURS: u64 = 0x0000_0003_0000_0042;

fn gone(device_id: u64) -> [u8; DGON_LEN] {
    dgon_encode(Dgon { why: WHY_GONE, device_id })
}

#[test]
fn a_dgon_for_this_device_ends_the_driver() {
    assert_eq!(turn(0, &gone(OURS), OURS), Turn::End);
}

#[test]
fn a_dgon_for_another_device_leaves_the_driver_running() {
    for other in [0, OURS ^ 1, OURS + (1 << 32), u64::MAX] {
        assert_eq!(turn(0, &gone(other), OURS), Turn::Drop, "{other:#x}");
    }
}

#[test]
fn any_other_frame_from_sender_0_is_dropped() {
    let (mut version, mut why) = (gone(OURS), gone(OURS));
    version[4] = 2;
    why[6] = 2;
    let long = [&gone(OURS)[..], &[0]].concat();
    let request = [0u8; 20];
    let frames: [&[u8]; 7] =
        [&[], &[0; DGON_LEN], &version, &why, &gone(OURS)[..15], &long, &request];
    for f in frames {
        assert_eq!(turn(0, f, OURS), Turn::Drop, "{f:?}");
    }
}

#[test]
fn a_dgon_sent_by_a_process_is_served_not_obeyed() {
    for sender in [1, 7, u32::MAX] {
        assert_eq!(turn(sender, &gone(OURS), OURS), Turn::Serve);
    }
}
