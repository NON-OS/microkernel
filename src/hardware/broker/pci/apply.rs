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

//! A judged config write made (L75). Run under the claims' lock while the
//! claim stands (L69). A Command write is judged again under the Command lock
//! against what the register holds then, so a bind's INTx Disable edit made
//! since the first read is kept, not undone (L73's one lock for every edit).

use super::types::{PciWriteError, WriteAction};
use crate::drivers::pci::config::ConfigSpace;

pub(super) fn apply(
    cfg: &ConfigSpace,
    action: WriteAction,
    judge: impl Fn(u16) -> Result<WriteAction, PciWriteError>,
) -> Result<(), PciWriteError> {
    match action {
        WriteAction::Command(_) => {
            let mut verdict = Ok(());
            // Refused now: the register is written back as it reads, which
            // changes nothing, since Command has no bit a write of 1 clears.
            let edit = |now: u16| match judge(now) {
                Ok(WriteAction::Command(value)) => value,
                Ok(_) => now,
                Err(e) => {
                    verdict = Err(e);
                    now
                }
            };
            cfg.update_command(edit).map_err(|_| PciWriteError::PlatformError)?;
            verdict
        }
        WriteAction::MsixControl { offset, value } | WriteAction::Bits { offset, value } => {
            cfg.write16(offset, value).map_err(|_| PciWriteError::PlatformError)
        }
    }
}
