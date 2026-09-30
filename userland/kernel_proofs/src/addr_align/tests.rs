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

use super::phys::PhysAddr;
use super::virt::VirtAddr;

#[test]
fn physical_rounding_lands_on_a_multiple() {
    for align in [1u64, 3, 4, 7, 12, 4096, 4097] {
        for addr in [0u64, 1, 10, 4095, 4096, 12_345, 1 << 40] {
            let down = PhysAddr::new(addr).align_down(align);
            let up = PhysAddr::new(addr).align_up(align);
            assert!(down.is_aligned(align), "{addr} down to {align}");
            assert!(up.is_aligned(align), "{addr} up to {align}");
            assert!(down.as_u64() <= addr && addr - down.as_u64() < align);
            assert!(up.as_u64() >= addr && up.as_u64() - addr < align);
        }
    }
    assert_eq!(PhysAddr::new(10).align_down(3).as_u64(), 9);
    assert_eq!(PhysAddr::new(10).align_up(3).as_u64(), 12);
}

#[test]
fn virtual_rounding_lands_on_a_multiple() {
    assert_eq!(VirtAddr::new(10).align_down(3).as_u64(), 9);
    assert_eq!(VirtAddr::new(10).align_up(3).as_u64(), 12);
    assert_eq!(VirtAddr::new(0x1234).align_down(4096).as_u64(), 0x1000);
    assert_eq!(VirtAddr::new(0x1234).align_up(4096).as_u64(), 0x2000);
}

#[test]
fn an_alignment_of_zero_leaves_the_address() {
    assert_eq!(PhysAddr::new(4097).align_down(0).as_u64(), 4097);
    assert_eq!(PhysAddr::new(4097).align_up(0).as_u64(), 4097);
    assert_eq!(VirtAddr::new(4097).align_down(0).as_u64(), 4097);
    assert_eq!(VirtAddr::new(4097).align_up(0).as_u64(), 4097);
}
