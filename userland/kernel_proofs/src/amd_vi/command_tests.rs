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

use super::command::{
    command_opcode, completion_wait, invalidate_all, invalidate_devtab_entry,
    invalidate_domain_pages,
};
use super::event::{address, code, device_id, domain, from_words, is_write, IO_PAGE_FAULT};
use super::regs::{dev_table_base, ring_base, ring_index, ring_next, ring_offset, RING_ENTRIES};

/* Linux build_completion_wait: data[0] = lower(paddr) | STORE (bit 0),
data[1] = upper(paddr) | type 1 << 28, data[2..3] = the 64-bit store data. */
#[test]
fn completion_wait_stores_its_sequence() {
    let c = completion_wait(0x1234_5678_9007, 0xDEAD_BEEF_0000_0001);
    assert_eq!(c, [0x5678_9001, 0x1000_1234, 0x0000_0001, 0xDEAD_BEEF]);
    assert_eq!(command_opcode(c), 1);
}

/* Linux build_inv_dte, build_inv_iommu_pages with the all-pages address
0x7FFF_FFFF_FFFF_F000 plus S and PDE, and build_inv_all. */
#[test]
fn invalidations_match_linux() {
    assert_eq!(invalidate_devtab_entry(0x00A0), [0xA0, 0x2000_0000, 0, 0]);
    assert_eq!(invalidate_domain_pages(5), [0, 0x3000_0005, 0xFFFF_F003, 0x7FFF_FFFF]);
    assert_eq!(invalidate_all(), [0, 0x8000_0000, 0, 0]);
}

#[test]
fn registers_hold_sizes_where_the_spec_puts_them() {
    assert_eq!(dev_table_base(0x4000_0000, 512), 0x4000_01FF);
    assert_eq!(dev_table_base(0x4000_0000, 1), 0x4000_0000);
    assert_eq!(ring_base(0x1234_5000), 0x1234_5000 | (8 << 56));
    assert_eq!(1u32 << 8, RING_ENTRIES as u32);
    for i in 0..RING_ENTRIES {
        assert_eq!(ring_index(ring_offset(i)), i);
    }
    assert_eq!(ring_next(RING_ENTRIES - 1), 0);
}

/* Linux iommu_print_event: devid event[0] 15:0, type event[1] 31:28, flags
event[1] 27:16 with RW = 0x020, address event[3]:event[2]. */
#[test]
fn a_page_fault_event_names_the_device_and_the_write() {
    let low = ((((2u32 << 28) | (0x020 << 16) | 5) as u64) << 32) | 0x0200;
    let high = (0x1u64 << 32) | 0x2345_6000;
    let e = from_words(low, high);
    assert_eq!(code(e), IO_PAGE_FAULT);
    assert_eq!(device_id(e), 0x0200);
    assert_eq!(domain(e), 5);
    assert!(is_write(e));
    assert_eq!(address(e), 0x1_2345_6000);
    assert_eq!(code(from_words(0, 0)), 0, "an unwritten entry");
}
