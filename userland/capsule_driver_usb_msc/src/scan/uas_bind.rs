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
//! A UAS device bound: its UAS alternate selected, command and status on
//! the first bulk pair and its data pipes on the second
//! (OP_CONFIGURE_BULK2). At SuperSpeed it moves on streams
//! (OP_CONFIGURE_STREAMS) on the status and data pipes, each command on
//! the stream its tag names; a SuperSpeed device that offers no streams is
//! left to BOT.

use nonos_uas::{uas_pipes, UasPipes};

use super::probe::Probe;
use super::ready_unit::finish;
use super::report::Stage;
use super::uas_setup::set_up;
use crate::disk::Disk;
use crate::state::State;
use crate::xhci::port_speed;

const SPEED_HIGH: u8 = 3;
const SPEED_SUPER: u8 = 4;
/// Streams asked for, as 2^n contexts: one command is in flight, so three
/// usable streams are plenty.
const STREAMS_LOG2: u8 = 2;

/// `None` when the device is not one to bind by UAS here.
pub(super) fn uas(xhci: u32, slot: u8, port: u8, desc: &[u8], state: &mut State) -> Option<Probe> {
    let p = uas_pipes(desc)?;
    let log2 = match port_speed(xhci, port)? {
        SPEED_HIGH => None,
        s if s >= SPEED_SUPER => Some(stream_log2(&p)?),
        _ => return None,
    };
    let data = match set_up(xhci, slot, desc, &p, log2) {
        Ok(data) => data,
        Err(e) => return Some(Probe::Failed(Stage::Configure, e)),
    };
    let mut disk = Disk::bound(xhci, slot, &data);
    (disk.uas, disk.streams) = (true, log2.is_some());
    Some(finish(disk, state))
}

/// The streams to ask for: as many as the status and data pipes all take,
/// up to STREAMS_LOG2; `None` when one takes none.
fn stream_log2(p: &UasPipes) -> Option<u8> {
    let n = [p.status, p.data_in, p.data_out].iter().map(|q| q.streams).min()?;
    (n > 0).then(|| n.min(STREAMS_LOG2))
}
