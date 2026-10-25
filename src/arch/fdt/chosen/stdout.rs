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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

impl<'a> Fdt<'a> {
    /// The console `/chosen` names, and the options after its `:` (such as
    /// `115200n8`), which may be empty. The path may be an alias, as the Pi's
    /// `serial0:115200n8` is.
    pub fn stdout(&self) -> Result<Option<(NodeRef<'a>, &'a [u8])>, FdtError> {
        let Some(chosen) = self.find_path(b"/chosen")? else {
            return Ok(None);
        };
        let prop = match self.property(&chosen, b"stdout-path")? {
            Some(p) => p,
            None => match self.property(&chosen, b"linux,stdout-path")? {
                Some(p) => p,
                None => return Ok(None),
            },
        };
        let value = prop.string()?;
        let (path, options) = match value.iter().position(|&b| b == b':') {
            Some(colon) => (&value[..colon], &value[colon + 1..]),
            None => (value, &[][..]),
        };
        // A console that is named but not found is an error: the caller
        // falls back to a default, and should say why.
        let node = self.resolve(path)?.ok_or(FdtError::NoSuchNode)?;
        Ok(Some((node, options)))
    }
}
