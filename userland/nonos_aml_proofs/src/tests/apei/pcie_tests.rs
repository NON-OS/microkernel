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

//! PCI Express sections, and sections not valid or not known.

use super::records::data;
use crate::apei::*;

#[test]
fn pcie_and_what_is_not_valid_or_known() {
    let fields: [(usize, &[u8]); 6] = [
        (24, &[0x86, 0x80]),
        (26, &[0x34, 0x12]),
        (31, &[3]),
        (32, &[0x1C]),
        (35, &[2]),
        (116, &[0x20, 0, 0, 0]),
    ];
    let pcie = data(208, 1 << 7, &fields);
    let r = record(&PCIE, &pcie);
    assert_eq!(r, Record::Pcie { uncorrectable: Some(0x20) }, "no ids or bus numbers kept");
    assert_eq!(record(&PCIE, &data(208, 0, &fields)), Record::Pcie { uncorrectable: None });
    assert_eq!(record(&PCIE, &pcie[..100]), Record::Other, "too short");
    assert_eq!(record(&[0; 16], &pcie), Record::Other);
    let mem = data(80, 0, &[(16, &[0xFF; 8])]);
    assert_eq!(record(&MEMORY, &mem), Record::Memory { kind: None }, "type not valid");
}
