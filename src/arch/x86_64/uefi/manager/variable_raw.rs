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

extern crate alloc;

use alloc::vec::Vec;

use super::core::UefiManager;
use crate::arch::x86_64::uefi::error::UefiError;
use crate::arch::x86_64::uefi::types::Guid;

impl UefiManager {
    /* The data alone, for callers that read the value and not its attributes. */
    pub(crate) fn read_variable_raw(&self, name: &str, guid: &Guid) -> Result<Vec<u8>, UefiError> {
        self.read_variable_with_attributes(name, guid).map(|(_, data)| data)
    }
}
