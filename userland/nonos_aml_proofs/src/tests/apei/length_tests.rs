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

//! Status blocks whose lengths run past what they hold.

use super::records::{block, entry};
use crate::apei::*;

#[test]
fn lengths_that_run_past() {
    assert!(parse_block(&block(0, 3, &[])).unwrap().is_empty());
    let one = entry(&MEMORY, FATAL, 0x300, &[0; 80]);
    let mut b = block(1, FATAL, &[one]);
    b.truncate(b.len() - 1);
    assert_eq!(parse_block(&b), None, "data length past the block");
    let mut lying = block(1, FATAL, &[entry(&MEMORY, FATAL, 0x300, &[0; 80])]);
    lying[20 + 24] = 0xFF;
    assert_eq!(
        parse_block(&lying).unwrap().entries.len(),
        0,
        "an entry past the data ends the walk"
    );
}
