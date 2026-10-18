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

//! A stall on a best-effort probe at bring-up (plan/50 C1). The command
//! times out and the device is reset; the queue is restarted on the same
//! grant and the primary surface still goes up on the 2D path, with virgl
//! off. The reset queue used to be handed on, so the surface was refused and
//! the attempt failed: a host whose virgl stalled lost 2D with it.

use nonos_libc::said;
use nonos_virtio::common::{DEVICE_FEATURE, DEVICE_STATUS, STATUS_FAILED};

use super::probe_stall::{probes_stalling_on, transport_up, NOTIFY, SHOWN};
use crate::constants::{VG_CMD_GET_CAPSET_INFO, VG_CMD_GET_EDID, VG_CMD_RESOURCE_UNREF};
use crate::setup::probes;
use crate::state::FenceCounter;

#[test]
fn a_stalled_edid_query_still_ends_with_the_2d_surface() {
    let (types, virgl_ready) = probes_stalling_on(VG_CMD_GET_EDID);
    let said = said();
    assert!(said.iter().any(|s| s.contains("[GPU-EDID] query failed: virtio-gpu: device timeout")));
    assert!(said.iter().any(|s| s.contains("control queue restarted")), "{said:?}");
    assert_eq!(types[0], VG_CMD_GET_CAPSET_INFO, "the 3D probe runs on the restarted queue");
    assert_eq!(types[types.len() - SHOWN.len()..], SHOWN);
    assert!(!virgl_ready);
}

#[test]
fn a_stall_on_any_probe_command_still_ends_with_the_2d_surface() {
    // Every command the probes send, from a run with no stall (no command
    // has type 0), but ATTACH_BACKING, which the 2D path sends too.
    let (all, _) = probes_stalling_on(0);
    let sent = &all[..all.len() - SHOWN.len()];
    assert_eq!(sent.first(), Some(&VG_CMD_GET_EDID));
    assert_eq!(sent.last(), Some(&VG_CMD_RESOURCE_UNREF), "the 3D probe ran to its release");
    for &step in sent.iter().filter(|&&t| t != SHOWN[1]) {
        let (types, virgl_ready) = probes_stalling_on(step);
        assert!(!types.contains(&step), "the stalled command was answered");
        assert_eq!(types[types.len() - SHOWN.len()..], SHOWN);
        assert!(!virgl_ready, "3D reported after a stall on {step:#x}");
    }
}

// A device that stalls and then refuses the features it was driven with
// cannot be brought back: the attempt fails by name, and the sequence
// resets the device and gives the claim back, as for any failed step.
#[test]
fn a_queue_that_cannot_be_restarted_fails_the_attempt_by_name() {
    let (h, features, part) = transport_up(VG_CMD_GET_EDID);
    h.regs.present32(DEVICE_FEATURE, 0);
    let probed = probes::run(h.queue, features, NOTIFY, &FenceCounter::new(), 1, 1);
    assert_eq!(probed.err(), Some("virtio: VERSION_1 not offered"));
    assert!(part.seen().is_empty(), "a command reached a device that settled no features");
    assert_eq!(h.regs.wrote8(DEVICE_STATUS) & STATUS_FAILED, STATUS_FAILED, "left FAILED");
}
