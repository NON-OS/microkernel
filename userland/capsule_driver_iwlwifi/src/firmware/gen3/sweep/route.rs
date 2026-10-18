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

//! Where each packet a sweep drains goes.

use core::cell::Cell;

use super::super::bringup::is_legacy;
use super::super::cmds::{REPLY_RX_MPDU_CMD, SCAN_COMPLETE_UMAC};
use super::super::dev::Flow;
use super::super::packet::Packet;
use super::super::rx_frame::{parse as parse_frame, RxFrame};
use super::super::scan::complete;

// Send each received management frame up and note the completion of the scan
// with `uid`; another scan's completion is not this one's.
pub(super) fn route(
    p: &Packet<'_>,
    on_frame: &mut dyn FnMut(RxFrame),
    frames: &mut u32,
    status: &Cell<Option<u8>>,
    uid: u32,
) -> Flow {
    if is_legacy(p, REPLY_RX_MPDU_CMD) {
        if let Some(f) = parse_frame(p.payload) {
            *frames = frames.saturating_add(1);
            on_frame(f);
        }
    } else if is_legacy(p, SCAN_COMPLETE_UMAC) {
        if let Some((got, s)) = complete(p.payload) {
            if got == uid {
                status.set(Some(s));
                return Flow::Done;
            }
        }
    }
    Flow::Continue
}
