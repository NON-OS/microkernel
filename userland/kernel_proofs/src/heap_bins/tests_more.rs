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

use super::bins::{Taken, CLASSES, KEEP};
use super::front::Front;
use super::tests::{block, heap};

#[test]
fn a_class_keeps_no_more_than_its_share() {
    let size = CLASSES[6];
    let (mut mem, _) = heap(KEEP / size + 1, size);
    let front = Front::new();
    for i in 0..KEEP / size {
        assert!(unsafe { front.push(6, block(&mut mem, size, i)) });
    }
    assert!(!unsafe { front.push(6, block(&mut mem, size, KEEP / size)) }, "kept past KEEP");
}

#[test]
fn a_link_overwritten_after_free_drops_the_list_instead_of_handing_it_out() {
    let (mut mem, ours) = heap(2, 64);
    let front = Front::new();
    let (a, b) = (block(&mut mem, 64, 0), block(&mut mem, 64, 1));
    unsafe { (front.push(0, a), front.push(0, b)) };
    unsafe { (b.add(8) as *mut usize).write(0x1000) };
    assert!(matches!(unsafe { front.pop(0, &ours) }, Taken::Broken));
    assert!(matches!(unsafe { front.pop(0, &ours) }, Taken::Empty), "the broken list survived");
}

#[test]
fn a_new_front_passes_no_size_over_in_the_first_region() {
    use core::sync::atomic::Ordering;
    assert_eq!(Front::new().first_short.load(Ordering::Relaxed), usize::MAX);
}
