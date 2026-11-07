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

//! The client's calls that name paths.

use super::client::{cat, Client, Listed, Totals, Transport};
use super::op;
use crate::errno::{Errno, EINVAL};
use crate::node::Attr;

impl<T: Transport> Client<T> {
    pub fn truncate(&mut self, path: &[u8], size: u64) -> Result<(), Errno> {
        self.ask(op::TRUNCATE, &size.to_le_bytes(), path).map(drop)
    }

    /// `flags` are Linux's RENAME_NOREPLACE (1) and RENAME_EXCHANGE (2).
    pub fn rename(&mut self, from: &[u8], to: &[u8], flags: u32) -> Result<(), Errno> {
        let len = u16::try_from(from.len()).map_err(|_| EINVAL)?;
        let fields = cat(&[&flags.to_le_bytes(), &len.to_le_bytes(), from]);
        self.ask(op::RENAME, &fields, to).map(drop)
    }

    pub fn unlink(&mut self, path: &[u8]) -> Result<(), Errno> {
        self.ask(op::UNLINK, &[], path).map(drop)
    }

    pub fn mkdir(&mut self, path: &[u8], mode: u32) -> Result<Attr, Errno> {
        super::client_read::attr(self.ask(op::MKDIR, &mode.to_le_bytes(), path)?)
    }

    pub fn rmdir(&mut self, path: &[u8]) -> Result<(), Errno> {
        self.ask(op::RMDIR, &[], path).map(drop)
    }

    /// Up to `max` names from the `cookie`-th.
    pub fn list(&mut self, path: &[u8], cookie: u64, max: u32) -> Result<Listed, Errno> {
        let body = self.ask(op::LIST, &cat(&[&cookie.to_le_bytes(), &max.to_le_bytes()]), path)?;
        super::client_read::listed(body)
    }

    /// `mask` is SET_MODE (1) and SET_MTIME (2).
    pub fn setattr(
        &mut self,
        path: &[u8],
        mask: u32,
        mode: u32,
        mtime: u64,
    ) -> Result<Attr, Errno> {
        let fields = cat(&[&mask.to_le_bytes(), &mode.to_le_bytes(), &mtime.to_le_bytes()]);
        super::client_read::attr(self.ask(op::SETATTR, &fields, path)?)
    }

    pub fn statfs(&mut self) -> Result<Totals, Errno> {
        super::client_read::totals(self.ask(op::STATFS, &[], &[])?)
    }
}
