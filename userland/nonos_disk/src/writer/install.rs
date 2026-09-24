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

//! The blocking install, for callers with no screen to keep alive: the
//! stepping session driven to the end in one call. The host tests use it;
//! the installer capsule drives the session itself.

use super::error::WriteError;
use super::receipt::Receipt;
use crate::image::NonosImage;
use crate::session::{Plan, Progress, Session};
use crate::sink::BlockSink;

/// Four MiB per step: a few hundred steps for the shipped image.
const STEP_BYTES: usize = 4 << 20;

/// `entropy` seeds the disk GUID, the partition GUID and the volume id.
pub fn install<'a>(
    sink: &mut dyn BlockSink,
    image: &NonosImage<'a>,
    entropy: [u8; 36],
    progress: &mut dyn FnMut(u64),
) -> Result<Receipt<'a>, WriteError> {
    let total = sink.capacity_sectors()?;
    let mut session = Session::new(Plan::new(total, image, entropy)?);
    let mut last = 0u64;
    loop {
        match session.step(sink, STEP_BYTES)? {
            Progress::Writing { done, .. } => {
                progress(done - last);
                last = done;
            }
            Progress::TableWritten => {}
            Progress::Done(receipt) => return Ok(receipt),
        }
    }
}
