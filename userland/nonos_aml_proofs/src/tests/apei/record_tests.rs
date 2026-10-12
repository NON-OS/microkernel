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

//! Error status blocks and their entries, and each section decoded to what
//! its validation bits allow.

use super::records::{block, data, entry};
use crate::apei::*;

#[test]
fn entries_with_and_without_a_timestamp() {
    let mem = data(
        80,
        1 << 1 | 1 << 14,
        &[(16, &0x1_2345_6000u64.to_le_bytes()), (32, &[1, 0]), (38, &[7, 0]), (72, &[2])],
    );
    let cpu = data(192, 1 << 2 | 1 << 5, &[(10, &[1]), (13, &[2])]);
    let b = block(
        0b10,
        CORRECTED,
        &[entry(&MEMORY, CORRECTED, 0x300, &mem), entry(&PROCESSOR, CORRECTED, 0x201, &cpu)],
    );
    let got = parse_block(&b).unwrap();
    assert!(!got.is_empty());
    assert_eq!(got.entries.len(), 2);
    let m = record(&got.entries[0].section, &got.entries[0].data);
    assert_eq!(
        m,
        Record::Memory {
            address: Some(0x1_2345_6000),
            node: 1,
            card: 0,
            module: 0,
            bank: 7,
            kind: 2
        }
    );
    let c = record(&got.entries[1].section, &got.entries[1].data);
    assert_eq!(
        c,
        Record::Processor { error_type: 1, level: 2, processor: None },
        "processor id not valid"
    );
}
