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

//! The one full walk every blob gets before it is trusted.

use super::error::FdtError;
use super::limits::MAX_DEPTH;
use super::walker::{Event, Walker};

/// Check what the walker alone does not: exactly one root, unnamed, with
/// every property inside a node and ahead of that node's subnodes. Cell
/// sizes are read from a parent's properties before its children are seen,
/// which is only sound when no property can follow a child.
pub(super) fn structure(structure: &[u8], strings: &[u8]) -> Result<(), FdtError> {
    let mut walker = Walker::new(structure, strings);
    let mut roots = 0usize;
    // Whether the node open at each depth has shown a subnode yet.
    let mut child_seen = [false; MAX_DEPTH + 1];
    while let Some(event) = walker.next_event()? {
        let depth = walker.depth();
        match event {
            Event::BeginNode { name, .. } => {
                if depth == 1 {
                    roots += 1;
                    if roots > 1 || !name.is_empty() {
                        return Err(FdtError::BadRoot);
                    }
                }
                child_seen[depth - 1] = true;
                child_seen[depth] = false;
            }
            Event::EndNode => {}
            Event::Property(_) if depth == 0 => return Err(FdtError::BadRoot),
            Event::Property(_) if child_seen[depth] => return Err(FdtError::PropertyAfterNode),
            Event::Property(_) => {}
        }
    }
    if roots != 1 {
        return Err(FdtError::BadRoot);
    }
    Ok(())
}
