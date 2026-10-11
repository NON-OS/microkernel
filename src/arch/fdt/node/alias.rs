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
use crate::arch::fdt::parser::Fdt;

use super::node_ref::NodeRef;

impl<'a> Fdt<'a> {
    /// A path that may start with an alias: `/soc/uart`, `serial0`, or
    /// `serial0/bluetooth`. An alias that names no path is `NoSuchNode`; one
    /// that is not in `/aliases` is simply absent.
    pub fn resolve(&self, path: &[u8]) -> Result<Option<NodeRef<'a>>, FdtError> {
        if path.first() == Some(&b'/') {
            return self.find_path(path);
        }
        let (alias, tail) = match path.iter().position(|&b| b == b'/') {
            Some(slash) => (&path[..slash], &path[slash..]),
            None => (path, &[][..]),
        };
        let Some(aliases) = self.find_path(b"/aliases")? else {
            return Ok(None);
        };
        let Some(target) = self.property(&aliases, alias)? else {
            return Ok(None);
        };
        let target = target.string()?;
        // An alias pointing at another alias is not allowed; only a path.
        if target.first() != Some(&b'/') {
            return Err(FdtError::NoSuchNode);
        }
        self.find_joined(target, tail)
    }
}
