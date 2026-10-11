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

//! A receive ring the driver has let fill.

use super::util::{drawn, frame_to, slot, up};
use crate::bench::Bench;
use crate::model::{with_part, Delivered, RS_WAITING};

// The part suspends its receive process with no descriptor to fill; the
// driver's poll demand on each recycle is what wakes it.
#[test]
fn a_full_ring_suspends_until_slots_go_back() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    for n in 0..32u8 {
        assert_eq!(with_part(|p| p.deliver(&frame_to(drawn(), 60, n), false)), Delivered::Taken);
    }
    assert_eq!(with_part(|p| p.deliver(&frame_to(drawn(), 60, 0xFF), false)), Delivered::NoBuffer);
    for n in 0..32u16 {
        let (idx, len) = d.rx.consume().expect("frame");
        assert_eq!((idx, len, slot(d.rx.pool_va, idx)[6]), (n, 60, n as u8));
        d.rx.recycle(idx);
    }
    assert_eq!(with_part(|p| p.rs), RS_WAITING, "woken by the poll demand");
    assert_eq!(with_part(|p| p.deliver(&frame_to(drawn(), 60, 7), false)), Delivered::Taken);
    assert_eq!(d.rx.consume(), Some((0, 60)), "the walk wrapped with the part");
}
