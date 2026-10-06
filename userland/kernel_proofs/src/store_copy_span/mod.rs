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

/*
 * Which store reads the loader's copy answers. Included by path, so the
 * arithmetic held here is the kernel's.
 */

#[allow(dead_code)]
#[path = "../../../../src/syscall/microkernel/store_copy_span.rs"]
mod span;

#[cfg(test)]
mod tests {
    use super::span::{held, offset, SECTOR, STORE_BASE_LBA};

    /* 3066560 bytes, the store a stick under OVMF carried: 5989 sectors and
     * 192 bytes, as the table of contents names it. */
    const NAMED: usize = 3_066_560;

    #[test]
    fn the_copy_runs_to_the_end_of_its_last_sector() {
        assert_eq!(held(NAMED), 5990 * SECTOR);
        assert_eq!(held(5990 * SECTOR), 5990 * SECTOR);
        assert_eq!(held(1), SECTOR);
    }

    #[test]
    fn the_last_entrys_final_sector_is_read_from_the_copy() {
        let last = STORE_BASE_LBA + 5989;
        assert_eq!(offset(last, SECTOR, NAMED), None, "counted to the byte, the copy missed it");
        assert_eq!(offset(last, SECTOR, held(NAMED)), Some(5989 * SECTOR));
        /* The whole-sector chunk vfs asks for, ending on that sector. */
        let chunk = 64 * SECTOR;
        assert_eq!(offset(last + 1 - 64, chunk, held(NAMED)), Some((5990 - 64) * SECTOR));
    }

    #[test]
    fn nothing_past_the_copy_or_before_the_store_is_answered() {
        let h = held(NAMED);
        assert_eq!(offset(STORE_BASE_LBA + 5990, SECTOR, h), None);
        assert_eq!(offset(STORE_BASE_LBA + 5989, 2 * SECTOR, h), None);
        assert_eq!(offset(STORE_BASE_LBA - 1, SECTOR, h), None);
        assert_eq!(offset(u64::MAX, SECTOR, h), None);
        assert_eq!(offset(STORE_BASE_LBA, SECTOR, h), Some(0));
    }
}
