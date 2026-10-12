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

//! One register access to a region.

use crate::exec::{Fault, FaultKind, Host, Machine, Region, RegionAccess};
use crate::load::Aml;
use crate::namespace::Id;

/// One access: `width` bytes at `byte` into the region, read, or written
/// with `write`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Io {
    pub byte: u64,
    pub width: u32,
    pub write: Option<u64>,
}

impl<H: Host> Machine<'_, H> {
    /// An access past the region's end is a fault, as is one to a space whose
    /// handler is not connected yet, or one the host refuses.
    pub(crate) fn region_io(
        &mut self,
        aml: &mut Aml,
        (rid, r): (Id, &Region),
        io: Io,
        at: u32,
    ) -> Result<u64, Fault> {
        let Io { byte, width, write } = io;
        if byte.checked_add(width as u64).is_none_or(|end| end > r.len) {
            return Err(self.fault(FaultKind::Index, at));
        }
        if let Some(t) = &r.table {
            let b = &t[byte as usize..(byte + width as u64) as usize];
            return match write {
                Some(_) => Err(self.fault(FaultKind::Region, at)),
                None => Ok(b.iter().rev().fold(0u64, |v, x| (v << 8) | *x as u64)),
            };
        }
        if aml.rt.connected & r.space.bit() == 0 {
            return Err(self.fault(FaultKind::NoHandler, at));
        }
        let a = RegionAccess {
            space: r.space,
            region: rid,
            address: r.base.wrapping_add(byte),
            width,
            pci: r.pci.as_ref(),
            base: r.base,
            len: r.len,
        };
        let done = match write {
            Some(v) => self.host.region_write(&a, v).then_some(v),
            None => self.host.region_read(&a),
        };
        done.ok_or_else(|| self.fault(FaultKind::Region, at))
    }
}
