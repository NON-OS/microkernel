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

//! The device's next batch while the stream runs.

use nonos_usbnet::scan::Bound;
use nonos_usbnet::xhci::E_INVAL;

use super::play::Play;
use crate::bind::Speaker;
use crate::xhci::isoch_out;

/// One batch: what is queued, then silence, so the device's clock does not
/// stop while the stream runs. isoch_out returns once the batch before has
/// played, which paces the run loop at the device's rate.
pub fn feed(play: &mut Play, b: &Bound<Speaker>, buf: &mut [u8]) -> Result<(), i32> {
    let t = b.nic.timing;
    let batch = buf.get_mut(..t.batch_bytes()).ok_or(E_INVAL)?;
    play.queue.pop_into(batch);
    isoch_out(b.xhci, b.slot, t.packet_bytes, t.packets, batch).map(|_| ())
}
