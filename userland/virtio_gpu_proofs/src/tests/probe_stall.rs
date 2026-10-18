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

//! The best-effort probes as bring-up runs them after the transport, on a
//! window offering VIRGL and EDID (with VERSION_1 and ACCESS_PLATFORM, see
//! `model`), against a part that leaves one type of command unanswered.

use nonos_virtio::common::{NotifyArea, DEVICE_FEATURE};

use super::frames::le32;
use super::harness::{harness, Harness};
use super::model::{bring_up, NOTIFY_LEN, NOTIFY_MULT};
use super::part::{answering_after_reset, Answer, Part};
use super::stall::PRIMARY;
use crate::constants::{VG_CMD_RESOURCE_ATTACH_BACKING, VG_CMD_RESOURCE_CREATE_2D};
use crate::constants::{VG_CMD_RESOURCE_FLUSH, VG_CMD_SET_SCANOUT, VG_CMD_TRANSFER_TO_HOST_2D};
use crate::setup::{prime, probes};
use crate::state::FenceCounter;

/// How the primary surface is shown: the 2D path.
pub const SHOWN: [u32; 5] = [CREATE, ATTACH, VG_CMD_TRANSFER_TO_HOST_2D, VG_CMD_SET_SCANOUT, FLUSH];
const CREATE: u32 = VG_CMD_RESOURCE_CREATE_2D;
const ATTACH: u32 = VG_CMD_RESOURCE_ATTACH_BACKING;
const FLUSH: u32 = VG_CMD_RESOURCE_FLUSH;

pub const NOTIFY: NotifyArea = NotifyArea { multiplier: NOTIFY_MULT, len: NOTIFY_LEN };

/// The queue as the transport hands it on, the features in force, and a
/// part that never answers a command of type `stalled`.
pub fn transport_up(stalled: u32) -> (Harness, u64, Part) {
    let h = harness();
    h.regs.present32(DEVICE_FEATURE, 3);
    let (features, _, _) = bring_up(&h.regs, NOTIFY_LEN).expect("a bring-up");
    let part = answering_after_reset(&h.region, &h.regs, Answer::StallOn(stalled), u16::MAX);
    (h, features, part)
}

/// The probes, then the primary shown on the queue they hand on: the type
/// of each command the part answered, and whether 3D was reported.
pub fn probes_stalling_on(stalled: u32) -> (Vec<u32>, bool) {
    let (h, features, part) = transport_up(stalled);
    let fences = FenceCounter::new();
    let probed = probes::run(h.queue, features, NOTIFY, &fences, 1, 1).expect("the queue back");
    let shown = prime::show(&probed.queue, &fences, &PRIMARY);
    assert_eq!(shown, Ok(()), "no 2D surface after a stall on {stalled:#x}");
    (part.seen().iter().map(|r| le32(r, 0)).collect(), probed.virgl_ready)
}
