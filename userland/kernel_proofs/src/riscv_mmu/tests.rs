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

use super::flags::PteFlags;
use super::mode::{MmuMode, KERNEL_MMU_MODE};

const NAMED: [MmuMode; 4] = [MmuMode::Bare, MmuMode::Sv39, MmuMode::Sv48, MmuMode::Sv57];

#[test]
fn an_unknown_mode_has_no_satp_encoding() {
    assert_eq!(MmuMode::Unknown.satp_mode(), None);
    for (i, a) in NAMED.iter().enumerate() {
        for b in &NAMED[i + 1..] {
            assert_ne!(a.satp_mode(), b.satp_mode());
        }
    }
    assert_eq!(KERNEL_MMU_MODE.satp_mode(), Some(8));
}

#[test]
fn leaf_is_valid_with_r_or_x_and_never_write_only() {
    for bits in 0..16u64 {
        let (v, r, w, x) = (bits & 1 != 0, bits & 2 != 0, bits & 4 != 0, bits & 8 != 0);
        let write_only = w && !r;
        let expected = v && (r || x) && !write_only;
        assert_eq!(PteFlags::from_bits(bits).is_leaf(), expected, "flags {bits:#x}");
    }
}
