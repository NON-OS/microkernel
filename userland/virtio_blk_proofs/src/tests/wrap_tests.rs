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

//! The driver's counts and the device's indices run past their 16-bit
//! wrap with every request still told apart from the one before it.

use super::model::{Rig, IRQ, SIZE};
use super::script::answer_each;
use crate::io::submit;
use crate::queue::{avail_offset, Direction};

#[test]
fn requests_go_on_being_matched_across_the_index_wrap() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    answer_each(&rig, 1);
    let total = 65_536 + 3 * u32::from(SIZE);
    for n in 0..total {
        let r = submit(rig.transport(), &mut q, IRQ, Direction::Flush, 0, 0);
        assert_eq!(r, Ok(()), "request {n}");
    }
    assert_eq!(rig.posted(), (total % 65_536) as u16);
    let last = avail_offset(SIZE) + 4 + 2 * ((total - 1) % u32::from(SIZE)) as usize;
    assert_eq!(rig.ring.wrote16(last), 0, "every entry names head 0");
    answer_each(&rig, 513);
    assert_eq!(submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1), Ok(()));
}
