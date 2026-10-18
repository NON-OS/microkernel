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

use super::super::distributor::GicDistributor;
use super::super::msi::{MsiError, Vector};
use super::super::state::dist_base;
use super::super::{configure_spi, disable_irq, enable_irq};
use super::state::V2M;

/// Hand out `out.len()` of the frame's SPIs, each set edge triggered as a
/// message is, and left disabled until `enable`.
pub fn bind(out: &mut [Vector]) -> Result<(), MsiError> {
    let mut guard = V2M.lock();
    let v2m = guard.as_mut().ok_or(MsiError::NotReady)?;
    for n in 0..out.len() {
        let taken = v2m.take().ok_or(MsiError::Exhausted).and_then(|intid| {
            configure_spi(intid, true).map(|()| intid).map_err(|_| {
                v2m.give(intid);
                MsiError::Hardware
            })
        });
        match taken {
            Ok(intid) => out[n] = Vector { intid, data: intid },
            Err(e) => {
                out[..n].iter().for_each(|v| v2m.give(v.intid));
                return Err(e);
            }
        }
    }
    Ok(())
}

pub fn enable(vectors: &[Vector]) {
    vectors.iter().for_each(|v| enable_irq(v.intid));
}

/// Disabled, and cleared of any message that landed after the device was
/// told to stop, before it is handed out again: the next owner's first
/// interrupt is its own.
pub fn unbind(vectors: &[Vector]) {
    let dist = GicDistributor::new(dist_base());
    let mut guard = V2M.lock();
    for v in vectors {
        disable_irq(v.intid);
        dist.clear_pending(v.intid);
        if let Some(v2m) = guard.as_mut() {
            v2m.give(v.intid);
        }
    }
}
