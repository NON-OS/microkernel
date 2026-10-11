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

use super::vendor::Hypervisor;

#[test]
fn a_corrupt_code_reads_as_unknown_not_as_bare_metal() {
    for code in 11..=255u8 {
        assert_eq!(Hypervisor::from_code(code), Hypervisor::Unknown);
    }
}

// The owner greps boot logs for these words; they are a contract, not prose.
#[test]
fn the_boot_line_slugs_and_leaves_are_fixed() {
    assert_eq!(super::leaves::LEAF_ALT, 0x4000_0100);
    let slugs: Vec<&str> = (0..=10u8).map(|c| Hypervisor::from_code(c).slug()).collect();
    let want = "none kvm qemu-tcg vmware hyperv virtualbox xen bhyve acrn parallels unknown";
    assert_eq!(slugs.join(" "), want);
}
