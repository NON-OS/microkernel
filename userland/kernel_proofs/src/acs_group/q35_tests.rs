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

//! K11b on QEMU q35 as the boot matrix runs it: every device on bus 0.

use super::acs_tree::Bdf;
use super::laptop_tests::reach;
use super::laptops::part;

#[test]
fn q35_as_the_boot_matrix_runs_it_lets_every_claim_go_ahead() {
    let ich9 = |f: u8, d: u16| part((0, 0x1f, f), None, (0x8086, d), None);
    let qemu = |slot: u8, ids: (u16, u16)| part((0, slot, 0), None, ids, Some(9));
    let parts = vec![
        ich9(0, 0x2918),
        ich9(2, 0x2922),
        ich9(3, 0x2930),
        qemu(1, (0x1AF4, 0x1050)),
        qemu(2, (0x1B36, 0x000D)),
        qemu(3, (0x1AF4, 0x1042)),
        qemu(4, (0x1B36, 0x0010)),
        part((0, 5, 0), None, (0x8086, 0x293E), None),
    ];
    let claimed = [(0, 0x1f, 2), (0, 1, 0), (0, 2, 0), (0, 3, 0), (0, 4, 0), (0, 5, 0)];
    for c in claimed {
        assert!(reach(&parts, c).iter().all(|(m, _)| !claimed.contains(m)), "{c:?}");
    }
    let ahci: Vec<Bdf> = reach(&parts, (0, 0x1f, 2)).into_iter().map(|(m, _)| m).collect();
    assert_eq!(ahci, [(0, 0x1f, 0), (0, 0x1f, 3)], "LPC and SMBus, held by nobody");
}
