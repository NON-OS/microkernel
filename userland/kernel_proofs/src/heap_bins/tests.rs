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

use super::bins::{class_of, Taken};
use super::front::Front;
use core::alloc::Layout;

/// A heap of `n` blocks of `size`, 16-aligned, and whether an address is in it.
pub(super) fn heap(n: usize, size: usize) -> (Vec<u128>, impl Fn(usize) -> bool) {
    let mem = vec![0u128; n * size / 16];
    let (start, len) = (mem.as_ptr() as usize, n * size);
    (mem, move |at: usize| at.wrapping_sub(start) < len)
}

pub(super) fn block(mem: &mut [u128], size: usize, i: usize) -> *mut u8 {
    unsafe { (mem.as_mut_ptr() as *mut u8).add(i * size) }
}

fn taken(t: Taken) -> Option<*mut u8> {
    match t {
        Taken::Block(b) => Some(b),
        Taken::Empty => None,
        Taken::Broken => panic!("a sound list read as broken"),
    }
}

#[test]
fn a_request_takes_the_least_class_that_holds_it_and_no_wide_alignment() {
    let at = |size, align| class_of(Layout::from_size_align(size, align).unwrap()).map(|c| c.0);
    assert_eq!(
        (at(40, 8), at(64, 16), at(65, 8), at(4096, 16)),
        (Some(0), Some(0), Some(1), Some(6))
    );
    assert_eq!((at(4097, 8), at(64, 32), at(8, 4096)), (None, None, None));
    let made = class_of(Layout::from_size_align(100, 8).unwrap()).unwrap().1;
    assert_eq!((made.size(), made.align()), (128, 16));
}

#[test]
fn freed_blocks_come_back_last_first_with_their_magic_cleared() {
    let (mut mem, ours) = heap(3, 64);
    let front = Front::new();
    let blocks: Vec<_> = (0..3).map(|i| block(&mut mem, 64, i)).collect();
    for &b in &blocks {
        unsafe { (b as *mut u64).write(0x4E4F_4E4F_4E4F_4E4F) };
        assert!(unsafe { front.push(0, b) });
        assert_eq!(unsafe { (b as *const u64).read() }, 0, "a freed block kept its magic");
    }
    for &b in blocks.iter().rev() {
        assert_eq!(taken(unsafe { front.pop(0, &ours) }), Some(b));
    }
    assert_eq!(taken(unsafe { front.pop(0, &ours) }), None);
}
