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

//! Giving the device back, which the driver does only before
//! its service is registered: the reset comes first, so the device reads
//! and writes nothing once the kernel tears the grants down.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use nonos_libc::{on_release, releases};
use nonos_virtio::common::DEVICE_STATUS;

use super::gpu::rig::rig;

#[test]
fn quiesce_resets_the_device_before_it_is_given_back() {
    let r = rig();
    r.regs.present8(DEVICE_STATUS, 0x0F);
    let (window, at_release) = (Arc::clone(&r.regs), Rc::new(Cell::new(None)));
    let (seen, before) = (Rc::clone(&at_release), releases());
    on_release(move || seen.set(Some(window.wrote8(DEVICE_STATUS))));
    assert!(r.d.quiesce(), "the reset read back zero");
    assert_eq!(releases() - before, 1, "given back once");
    assert_eq!(at_release.get(), Some(0), "the device was live when it was given back");
}
