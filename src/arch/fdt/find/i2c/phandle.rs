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

//! The node a phandle names, and a fixed clock's rate from it.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::walker::Event;

use super::seen::{Provider, Seen};

/// A node's properties all come before its children, so they are complete
/// at the next node boundary; the node is judged there.
pub(super) fn provider(fdt: &Fdt, phandle: u32) -> Result<Option<Provider>, FdtError> {
    let mut walker = fdt.walker();
    let mut seen: Option<Seen> = None;
    while let Some(event) = walker.next_event()? {
        if let Event::Property(p) = event {
            if let Some(s) = seen.as_mut() {
                s.see(&p);
            }
            continue;
        }
        if let Some(s) = seen.take().filter(|s| s.phandle == Some(phandle)) {
            return Ok(Some(s.provider()));
        }
        if matches!(event, Event::BeginNode { .. }) {
            seen = Some(Seen::default());
        }
    }
    Ok(None)
}
