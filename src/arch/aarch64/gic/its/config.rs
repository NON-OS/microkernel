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

use super::super::intid::{is_lpi, LPI_BASE};
use super::super::msi::{MsiError, Vector};
use super::encode::cmd::{inv, Cmd};
use super::encode::lpi::config;
use super::memory::{clean, view};
use super::state::{with, Its};
use super::vectors::MAX_VECTORS;

/// Turn the LPIs on: each configuration byte, then an INV so the
/// redistributor reloads it.
pub fn enable(device_id: u32, vectors: &[Vector]) -> Result<(), MsiError> {
    if vectors.len() > MAX_VECTORS {
        return Err(MsiError::Exhausted);
    }
    with(|its| {
        its.configure(vectors, true)?;
        let mut cmds: [Cmd; MAX_VECTORS] = [[0; 4]; MAX_VECTORS];
        for (cmd, v) in cmds.iter_mut().zip(vectors) {
            *cmd = inv(device_id, v.data);
        }
        its.queue.run(its.regs, &cmds[..vectors.len()])
    })
}

impl Its {
    /// Write each vector's configuration byte, cleaned out to memory for a
    /// redistributor that does not snoop. Takes effect at the next INV, or
    /// at once for an LPI no translation names any more.
    pub(super) fn configure(&mut self, vectors: &[Vector], enabled: bool) -> Result<(), MsiError> {
        let table = view(&self.lpi.prop).ok_or(MsiError::Hardware)?;
        for v in vectors.iter().filter(|v| is_lpi(v.intid)) {
            let at = (v.intid - LPI_BASE) as u64;
            table[at as usize] = config(enabled);
            if !self.lpi.coherent {
                clean(&self.lpi.prop, at, 1);
            }
        }
        Ok(())
    }
}
