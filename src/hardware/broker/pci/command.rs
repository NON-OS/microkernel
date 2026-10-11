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

//! The PCI Command register half of the `MkPciConfigWrite` allowlist.

use super::command_rule::{command_value, Refused};
use super::types::{PciWriteError, WriteAction};

/// The Command register write, by the rule in command_rule.rs.
pub(super) fn validate_command(
    new: u16,
    current: u16,
    may_master: bool,
) -> Result<WriteAction, PciWriteError> {
    match command_value(new, current, may_master) {
        Ok(value) => Ok(WriteAction::Command(value)),
        Err(Refused::Bits) => Err(PciWriteError::BitsNotAllowed),
        Err(Refused::BusMaster) => Err(PciWriteError::BusMasterWithoutDma),
    }
}
