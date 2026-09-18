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

//! The blocking read-back: the stepping verifier driven to the end. A write
//! the driver acknowledged is not a write that landed; the medium, the
//! controller and the cable all sit between, and the only evidence that the
//! disk holds the image is the disk saying so.

use super::error::WriteError;
use super::receipt::Receipt;
use crate::session::Verifier;
use crate::sink::BlockSink;

const STEP_BYTES: usize = 1 << 20;

/// Bytes checked, or the first sector that disagreed.
pub fn verify(
    sink: &mut dyn BlockSink,
    receipt: &Receipt<'_>,
    progress: &mut dyn FnMut(u64),
) -> Result<u64, WriteError> {
    let mut v = Verifier::new(receipt);
    let mut last = 0u64;
    while v.step(sink, STEP_BYTES)? {
        progress(v.checked - last);
        last = v.checked;
    }
    Ok(v.checked)
}
