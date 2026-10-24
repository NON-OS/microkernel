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

//! A completion whose flags octet cannot be read is refused, not read as a
//! zero: a zero there would take one piece of a multi-buffer frame as whole.

use std::cell::Cell;
use std::rc::Rc;

use crate::gen3::plan::RX_USED;
use crate::gen3::region::Region;
use crate::gen3::rxq::{RxQueue, Taken};
use crate::gen3_model::Mem;

use super::regions::regions;
use super::rxq::complete_at;

/// A region that fails the one read covering `fail_at`, and is otherwise `Mem`.
struct Failing {
    mem: Rc<Mem>,
    fail_at: Cell<Option<usize>>,
}

impl Region for Failing {
    fn len(&self) -> usize {
        self.mem.len()
    }
    fn dev(&self) -> u64 {
        self.mem.dev()
    }
    fn write(&self, off: usize, src: &[u8]) -> bool {
        self.mem.write(off, src)
    }
    fn read(&self, off: usize, dst: &mut [u8]) -> bool {
        match self.fail_at.get() {
            Some(a) if off <= a && a < off + dst.len() => false,
            _ => self.mem.read(off, dst),
        }
    }
}

const FRAGMENTED: u8 = 1;

#[test]
fn a_completion_whose_flags_cannot_be_read_is_a_bad_entry() {
    let (ctrl_mem, rbs_mem) = regions();
    let ctrl = Failing { mem: ctrl_mem, fail_at: Cell::new(None) };
    let rbs = Failing { mem: rbs_mem, fail_at: Cell::new(None) };
    let mut q = RxQueue::new();
    assert!(q.init(&ctrl, &rbs));
    // The device wrote a fragment, but its flags octet cannot be read.
    complete_at(ctrl.mem.as_ref(), 0, 3, FRAGMENTED, 1);
    ctrl.fail_at.set(Some(RX_USED + 6));
    let mut out = [0u8; 2048];
    assert_eq!(q.take(&ctrl, &rbs, &mut out), Some(Taken::BadId));
}
