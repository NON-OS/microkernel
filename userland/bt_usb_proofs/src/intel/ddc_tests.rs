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

use super::loader::carried::carried;
use super::loader::ddc::entries;
use super::rig::{bootloader, run, SOLAR};
use super::rig_ops::ops_after;

#[test]
fn ddc_entries_are_a_length_then_that_many_octets() {
    let two = [3, 0x28, 0x01, 0x18, 4, 0x29, 0x01, 0x03, 0x00];
    assert_eq!(entries(&two).unwrap(), [&two[..4], &two[4..]]);
    assert!(entries(&two[..3]).is_none(), "an entry past the end");
    assert_eq!(entries(&[]).unwrap().len(), 0);
}

#[test]
fn the_booted_controller_takes_the_carried_ddc_one_entry_a_command() {
    let (r, m, _) = run(bootloader(SOLAR, 1), 0x0033);
    assert_eq!(r, Ok(()));
    let ddc = carried(SOLAR.3).unwrap().ddc;
    assert_eq!(m.ddc, entries(ddc).unwrap());
    assert_eq!(m.ddc.concat(), ddc);
    assert_eq!(ops_after(&m, 0xfc01), [0xfc05, 0xfc8b, 0xfc8b, 0xfc52, 0x0c03]);
}
