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

//! Command register edits that cannot lose each other's bits (L73). An edit
//! reads Command and writes it back changed; two made at once on one
//! function each keep what the other changed between its read and its
//! write, so an exit turning Bus Master off and a bind setting INTx Disable
//! on another CPU could leave Bus Master on. Every edit the broker makes
//! goes through this one lock.

use spin::Mutex;

use super::super::constants::CFG_COMMAND;
use super::super::error::Result;
use super::config_space::ConfigSpace;

static COMMAND: Mutex<()> = Mutex::new(());

impl ConfigSpace {
    /// Command set to `edit` of what it holds, read and written under one
    /// lock; the value written.
    pub fn update_command(&self, edit: impl FnOnce(u16) -> u16) -> Result<u16> {
        let _one = COMMAND.lock();
        let value = edit(self.read16(CFG_COMMAND)?);
        self.write16(CFG_COMMAND, value)?;
        Ok(value)
    }
}
