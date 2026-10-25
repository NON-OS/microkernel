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

//! K15b: an orphan's holders are counted against its own owner thread,
//! never against the thread whose grant is being released.

use super::common::{grant_frames, Registry, FB, GPU, LEN, START};
use super::dma_rule::PageOrphan;

const WORKER: u32 = 8;

#[test]
fn the_owners_record_on_its_orphan_is_no_foreign_holder() {
    let mut r =
        Registry { attaches: vec![(FB, GPU)], threads: vec![(WORKER, GPU)], ..Default::default() };
    r.orphans.push(PageOrphan { owner: GPU, handle: FB, frames: grant_frames(), free: Vec::new() });
    assert!(!r.release(WORKER, Vec::new()), "only its owner maps it: freed at once");
    assert_eq!(r.given_back, vec![(START, LEN)]);
}
