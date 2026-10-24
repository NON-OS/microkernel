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

//! The ID register at every value it can hold: the adapter is driven only
//! when it names a DISPI version from 0xB0C0 to 0xB0C5.

use nonos_devmodel::FakeBar;

use crate::constants::DISPI_INDEX_ID;
use crate::dispi::{dispi_off, identify};
use crate::error::BgaError;
use crate::regs::Regs;

#[test]
fn every_id_value_is_driven_only_when_a_dispi_version() {
    let bar = FakeBar::new(0x600);
    for id in 0..=u16::MAX {
        bar.present16(dispi_off(DISPI_INDEX_ID) as usize, id);
        let want = if (0xB0C0..=0xB0C5).contains(&id) { Ok(()) } else { Err(BgaError::NotDispi) };
        assert_eq!(identify(Regs::new(bar.base())), want, "id {id:#06x}");
    }
}
