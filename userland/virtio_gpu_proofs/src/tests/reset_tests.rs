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
//! The reset a bring-up starts from (G-73): status 0 written, and nothing
//! more until the device has finished it and the status reads 0, as a
//! device that drops what a dead driver left must be let to. The device
//! takes its turns on the driver's yields: one still resetting keeps the
//! status up, and one that never finishes has the bring-up give up.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{off_yield, on_yield};

use crate::constants::{
    LEG_QUEUE_PFN, LEG_STATUS, MOD_DEVICE_STATUS, MOD_QUEUE_DESC, MOD_QUEUE_ENABLE,
    VIRTIO_GPU_F_EDID, VIRTIO_GPU_F_VIRGL, VIRTIO_GPU_MODERN, VIRTIO_GPU_TRANSITIONAL, VQ_MAX_SIZE,
};
use crate::init::bring_up;
use crate::regs::Regs;
use crate::tests::model::{legacy_window, modern_regs, modern_window, QUEUE_SIZE, REGION_PHYS};

/// The device answers the status byte with what a live part shows for the
/// first `busy` turns after the reset was asked, then with 0.
fn resetting(bar: &Arc<FakeBar>, at: usize, busy: u32) {
    let (bar, mut turns) = (Arc::clone(bar), 0);
    on_yield(move || {
        if turns < busy {
            bar.present8(at, 0x0F);
        } else if turns == busy {
            bar.present8(at, 0);
        }
        turns += 1;
    });
}

#[test]
fn bring_up_waits_for_the_reset_to_finish() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL | VIRTIO_GPU_F_EDID, QUEUE_SIZE);
    resetting(&bar, MOD_DEVICE_STATUS, 40);
    let up = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN);
    off_yield();
    assert!(up.is_ok(), "live once the reset finished");
    assert_eq!(bar.wrote8(MOD_DEVICE_STATUS), 0x0F);
}

#[test]
fn a_reset_that_never_finishes_has_nothing_else_written() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL | VIRTIO_GPU_F_EDID, QUEUE_SIZE);
    resetting(&bar, MOD_DEVICE_STATUS, u32::MAX);
    let up = bring_up(modern_regs(&bar), REGION_PHYS, VIRTIO_GPU_MODERN);
    off_yield();
    assert_eq!(up.err(), Some("virtio-gpu: device reset did not complete"));
    assert_eq!(bar.wrote8(MOD_DEVICE_STATUS), 0x0F, "as the device left it");
    assert_eq!((bar.wrote32(MOD_QUEUE_DESC), bar.wrote16(MOD_QUEUE_ENABLE)), (0, 0));
    let legacy = legacy_window(0, VQ_MAX_SIZE);
    resetting(&legacy, LEG_STATUS, u32::MAX);
    let up = bring_up(Regs::mmio(legacy.base()), REGION_PHYS, VIRTIO_GPU_TRANSITIONAL);
    off_yield();
    assert_eq!(up.err(), Some("virtio-gpu: device reset did not complete"));
    assert_eq!(legacy.wrote32(LEG_QUEUE_PFN), 0, "no ring published");
}
