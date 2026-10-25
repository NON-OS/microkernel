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

//! Sense data in either format, and a CHECK CONDITION with none.

use crate::ufs::error::UfsError;
use crate::ufs::scsi::sense::parse;

#[test]
fn fixed_and_descriptor_sense_and_none() {
    let mut fixed = [0u8; 18];
    fixed[0] = 0xF0;
    fixed[2] = 0x26;
    fixed[12] = 0x29;
    fixed[13] = 0x01;
    assert_eq!(
        parse(&fixed),
        UfsError::Sense { key: 0x6, asc: 0x29, ascq: 0x01 },
        "valid bit and high nibble ignored"
    );
    assert_eq!(parse(&[0x72, 0x05, 0x24, 0x00]), UfsError::Sense { key: 0x5, asc: 0x24, ascq: 0 });
    assert_eq!(parse(&[]), UfsError::Status(0x02));
    assert_eq!(parse(&fixed[..13]), UfsError::Status(0x02), "fixed sense cut before ASCQ");
    assert_eq!(parse(&[0x7F, 1, 2, 3]), UfsError::Status(0x02), "a vendor format");
}
