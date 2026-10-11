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
    /// Which of `needles` the node is compatible with, as an index into them.
    /// The node's own list runs most specific first, so its first string that
    /// appears in `needles` decides.
    pub fn compatible_with(
        &self,
        node: &NodeRef,
        needles: &[&[u8]],
    ) -> Result<Option<usize>, FdtError> {
        let Some(compat) = self.property(node, b"compatible")? else {
            return Ok(None);
        };
        Ok(compat.stringlist().find_map(|s| needles.iter().position(|n| *n == s)))
    }

    /// Every enabled node compatible with one of `needles`, with the index of
    /// the needle it matched.
    pub fn devices<'n>(
        &'n self,
        needles: &'n [&'n [u8]],
    ) -> impl Iterator<Item = Result<(NodeRef<'a>, usize), FdtError>> + 'n {
        self.nodes().filter_map(move |node| {
            let node = match node {
                Ok(node) => node,
                Err(e) => return Some(Err(e)),
            };
            match (self.compatible_with(&node, needles), self.is_enabled(&node)) {
                (Ok(Some(index)), Ok(true)) => Some(Ok((node, index))),
                (Err(e), _) | (_, Err(e)) => Some(Err(e)),
                _ => None,
            }
        })
    }
}
