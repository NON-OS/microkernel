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

use super::super::{its, v2m};
use super::controller::{kind, ITS, V2M};
use super::error::MsiError;
use super::vector::Vector;

/// The most vectors one bind takes.
pub const MAX_VECTORS: usize = its::MAX_VECTORS;

/// Hand out `out.len()` vectors for `device_id`, left disabled until
/// `enable`, so the caller can register their handlers and program the
/// device first.
pub fn bind(device_id: u32, out: &mut [Vector]) -> Result<(), MsiError> {
    if out.is_empty() || out.len() > MAX_VECTORS {
        return Err(MsiError::Exhausted);
    }
    match kind() {
        ITS => its::bind(device_id, out),
        V2M => v2m::bind(out),
        _ => Err(MsiError::NotReady),
    }
}

pub fn enable(device_id: u32, vectors: &[Vector]) -> Result<(), MsiError> {
    match kind() {
        ITS => its::enable(device_id, vectors),
        V2M => Ok(v2m::enable(vectors)),
        _ => Err(MsiError::NotReady),
    }
}

/// Take back what `bind` handed out, once the device has been told to stop
/// writing. An error means the controller may still deliver them, and the
/// caller keeps them out of use.
pub fn unbind(device_id: u32, vectors: &[Vector]) -> Result<(), MsiError> {
    match kind() {
        ITS => its::unbind(device_id, vectors),
        V2M => Ok(v2m::unbind(vectors)),
        _ => Err(MsiError::NotReady),
    }
}
