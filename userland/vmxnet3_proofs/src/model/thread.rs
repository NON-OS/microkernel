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
// MERCHANTABILITY or FITNESS FOR A DEVICEICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! One device per test thread, reached by the driver's register pages.

use std::cell::RefCell;

use super::device::Device;

thread_local! {
    static DEVICE: RefCell<Option<Device>> = const { RefCell::new(None) };
}

pub fn install(device: Device) {
    DEVICE.with(|p| *p.borrow_mut() = Some(device));
}

/// The device, for a test to inspect after the driver is done with it.
pub fn take() -> Device {
    DEVICE.with(|p| p.borrow_mut().take()).expect("no device installed on this thread")
}

pub fn with_device<R>(f: impl FnOnce(&mut Device) -> R) -> R {
    DEVICE.with(|p| f(p.borrow_mut().as_mut().expect("no device installed on this thread")))
}
