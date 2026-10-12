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

use core::sync::atomic::AtomicBool;
use spin::Mutex;

use super::device::VirtioRngDevice;

pub(super) static VIRTIO_RNG: Mutex<Option<VirtioRngDevice>> = Mutex::new(None);
/// Set only once the device has delivered bytes, and cleared when it goes to
/// its capsule: entropy init counts it as a verified source on this word.
pub(super) static VIRTIO_RNG_AVAILABLE: AtomicBool = AtomicBool::new(false);
