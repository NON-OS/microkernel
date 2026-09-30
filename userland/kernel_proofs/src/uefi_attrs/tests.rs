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

use crate::arch::x86_64::uefi::types::VariableAttributes;

#[test]
fn each_authenticated_write_flag_needs_authentication() {
    for flag in [
        VariableAttributes::AUTHENTICATED_WRITE_ACCESS,
        VariableAttributes::TIME_BASED_AUTHENTICATED_WRITE_ACCESS,
        VariableAttributes::ENHANCED_AUTHENTICATED_ACCESS,
    ] {
        assert!(flag.requires_authentication(), "{:#x}", flag.bits());
        let word = VariableAttributes::from_bits(
            VariableAttributes::DEFAULT_NV_BS_RT.bits() | flag.bits(),
        );
        assert!(word.requires_authentication(), "{:#x}", word.bits());
    }
    assert!(!VariableAttributes::DEFAULT_NV_BS_RT.requires_authentication());
    assert!(!VariableAttributes::from_bits(0x4F).requires_authentication());
}
