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

//! The node a phandle names, and what a controller needs from it: the
//! interrupt specifier width, and a fixed clock's rate.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::property::Property;
use crate::arch::fdt::walker::Event;

const FIXED_CLOCK: &[u8] = b"fixed-clock";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Provider {
    pub interrupt_cells: Option<u32>,
    pub fixed_hz: Option<u32>,
}

#[derive(Default)]
struct Seen {
    phandle: Option<u32>,
    fixed: bool,
    hz: Option<u32>,
    interrupt_cells: Option<u32>,
}

/// A node's properties all come before its children, so they are complete
/// at the next node boundary; the node is judged there.
pub(super) fn provider(fdt: &Fdt, phandle: u32) -> Result<Option<Provider>, FdtError> {
    let mut walker = fdt.walker();
    let mut seen: Option<Seen> = None;
    while let Some(event) = walker.next()? {
        if let Event::Property(p) = event {
            if let Some(s) = seen.as_mut() {
                see(s, &p);
            }
            continue;
        }
        if let Some(s) = seen.take().filter(|s| s.phandle == Some(phandle)) {
            let fixed_hz = s.hz.filter(|_| s.fixed);
            return Ok(Some(Provider { interrupt_cells: s.interrupt_cells, fixed_hz }));
        }
        if matches!(event, Event::BeginNode { .. }) {
            seen = Some(Seen::default());
        }
    }
    Ok(None)
}

fn see(s: &mut Seen, p: &Property) {
    match p.name {
        b"phandle" => s.phandle = be_u32(p.data, 0).ok(),
        b"compatible" => s.fixed = p.compatible_matches(FIXED_CLOCK),
        b"clock-frequency" => s.hz = be_u32(p.data, 0).ok(),
        b"#interrupt-cells" => s.interrupt_cells = be_u32(p.data, 0).ok(),
        _ => {}
    }
}
