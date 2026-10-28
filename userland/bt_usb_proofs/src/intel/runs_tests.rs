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

use super::loader::cmds::{boot_params, runs};

#[test]
fn commands_are_cut_only_on_aligned_whole_runs() {
    let nop = [0x02, 0xfc, 0x01, 0x00];
    assert_eq!(runs(&nop).unwrap(), [&nop[..]]);
    let two = [0x02, 0xfc, 0x00, 0x02, 0xfc, 0x02, 0, 0];
    assert_eq!(runs(&two).unwrap(), [&two[..]], "3 + 5 octets go together");
    assert!(runs(&nop[..3]).is_none(), "three octets end off the edge");
    assert!(runs(&[0x02, 0xfc, 0x09, 0]).is_none(), "a command past the end");
    assert_eq!(boot_params(&nop), None);
}
