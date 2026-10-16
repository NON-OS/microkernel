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

//! A device as a dead predecessor left it, and the window bring-up maps
//! over it.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_virtio::common::{DEVICE_STATUS, QUEUE_DESC, QUEUE_ENABLE};
use nonos_virtio::map::{Mapped, Window};

use super::harness::HANDSHAKE;
use super::model::{
    modern_window, view, DEVICE_AT, NOTIFY_AT, NOTIFY_LEN, NOTIFY_MULT, QUEUE_SIZE,
};

/// Where the predecessor's descriptor table was.
const STALE_RING: u32 = 0x00BA_D000;

/// The window as `setup::mmio` maps it from the three capabilities.
pub fn window(bar: &FakeBar) -> Window {
    let at = |at, len| Mapped { grant_id: 1, region: view(bar, at, len) };
    let (common, notify, device) =
        (at(0, NOTIFY_AT), at(NOTIFY_AT, NOTIFY_LEN), at(DEVICE_AT, 0x100));
    Window { common, notify, notify_multiplier: NOTIFY_MULT, isr: None, device: Some(device) }
}

/// A device a predecessor left live: DRIVER_OK, its control queue enabled.
pub fn left_live() -> Arc<FakeBar> {
    let bar = modern_window(1, QUEUE_SIZE);
    bar.present8(DEVICE_STATUS, HANDSHAKE);
    bar.present16(QUEUE_ENABLE, 1);
    bar.present32(QUEUE_DESC, STALE_RING);
    bar
}
