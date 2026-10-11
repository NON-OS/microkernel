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

//! Routing a device's events to LPIs, enabling them, and taking them back.

use super::super::msi::{MsiError, Vector};
use super::encode::cmd::{mapti, Cmd};
use super::slots::{discards, forget, take};
use super::state::{with, COLLECTION, EVENT_BITS};

/// The most vectors one bind takes: every event a device has.
pub const MAX_VECTORS: usize = 1 << EVENT_BITS;

/// Route `out.len()` fresh events of `device_id` to fresh LPIs, left
/// disabled until `enable`, and fill `out` with them.
pub fn bind(device_id: u32, out: &mut [Vector]) -> Result<(), MsiError> {
    if out.is_empty() || out.len() > MAX_VECTORS {
        return Err(MsiError::Exhausted);
    }
    with(|its| {
        let at = its.device(device_id)?;
        let mut cmds: [Cmd; MAX_VECTORS] = [[0; 4]; MAX_VECTORS];
        for n in 0..out.len() {
            let Some(v) = take(its, at) else {
                forget(its, at, &out[..n]);
                return Err(MsiError::Exhausted);
            };
            out[n] = v;
            cmds[n] = mapti(device_id, v.data, v.intid, COLLECTION);
        }
        if let Err(e) = its.queue.run(its.regs, &cmds[..out.len()]) {
            // Some may be in place: discard before anything is reused.
            if its.queue.run(its.regs, discards(device_id, out, &mut cmds)).is_ok() {
                forget(its, at, out);
            }
            return Err(e);
        }
        Ok(())
    })
}

/// Undo `bind`: once the ITS has discarded every translation, and any
/// interrupt one left pending, the LPIs and events are free again. Until it
/// confirms that, they stay taken, since a reused LPI could still fire.
pub fn unbind(device_id: u32, vectors: &[Vector]) -> Result<(), MsiError> {
    if vectors.len() > MAX_VECTORS {
        return Err(MsiError::Exhausted);
    }
    with(|its| {
        let at = its.find(device_id).ok_or(MsiError::DeviceOutOfRange)?;
        let mut cmds: [Cmd; MAX_VECTORS] = [[0; 4]; MAX_VECTORS];
        its.queue.run(its.regs, discards(device_id, vectors, &mut cmds))?;
        its.configure(vectors, false)?;
        forget(its, at, vectors);
        its.retire(at);
        Ok(())
    })
}
