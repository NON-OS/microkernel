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

//! Each recovery try is a whole bring-up of the queue on the same grant:
//! the device reset, ACKNOWLEDGE and DRIVER, the features taken against
//! those offered and FEATURES_OK read back, then the queue and DRIVER_OK.
//! A try that only reprogrammed the queue would drive a device that had
//! settled no features and never said FEATURES_OK.

use nonos_libc::{releases, said};
use nonos_virtio::common::{DEVICE_FEATURE, DEVICE_STATUS};

use super::harness::HANDSHAKE;
use super::part::{answering_after_reset, Answer};
use super::stall::{handle, stalled, PRIMARY};
use crate::constants::VIRTIO_GPU_F_VIRGL;
use crate::server::recover::settle;
use crate::state::Health;

#[test]
fn every_recovery_try_runs_the_whole_status_handshake() {
    let (mut d, region, regs) = stalled();
    // Each try reaches the part once: it refuses the try's first command.
    let part = answering_after_reset(&region, &regs, Answer::Reject, u16::MAX);
    settle(&mut d);
    assert_eq!(d.health, Health::Down);
    assert_eq!(part.statuses(), [HANDSHAKE; 3], "a try drove the device short of the handshake");
    // The window offers VIRGL; the harness's driver started with none.
    assert_eq!(d.host_features, VIRTIO_GPU_F_VIRGL, "the features in force, taken again");
}

#[test]
fn a_device_that_drops_version_1_after_the_reset_is_never_driven() {
    let (mut d, region, regs) = stalled();
    d.virgl_ready = true;
    let part = answering_after_reset(&region, &regs, Answer::Spec, u16::MAX);
    regs.present32(DEVICE_FEATURE, 0);
    settle(&mut d);
    assert_eq!(d.health, Health::Down, "a try ran on features never settled");
    assert!(part.seen().is_empty(), "a command reached a device that settled no features");
    assert_eq!(regs.wrote8(DEVICE_STATUS), 0, "the device is left reset");
    assert!(!d.virgl_ready, "the reset took render context 1");
    assert_eq!(said().len(), 4, "three tries and the state: {:?}", said());
    assert_eq!(handle(&d), Some(PRIMARY.handle), "the surface is still served");
    assert_eq!(releases(), 0);
}
