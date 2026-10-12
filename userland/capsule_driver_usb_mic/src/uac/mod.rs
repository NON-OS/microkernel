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
#[path = "../../../capsule_driver_usb_audio/src/uac/alt.rs"]
mod alt;
#[path = "../../../capsule_driver_usb_audio/src/uac/clock.rs"]
mod clock;
#[path = "../../../capsule_driver_usb_audio/src/uac/entities.rs"]
mod entities;
#[path = "../../../capsule_driver_usb_audio/src/uac/find.rs"]
mod find;
#[path = "../../../capsule_driver_usb_audio/src/uac/format.rs"]
mod format;
#[path = "../../../capsule_driver_usb_audio/src/uac/pick.rs"]
mod pick;
#[path = "../../../capsule_driver_usb_audio/src/uac/read.rs"]
mod read;
#[path = "../../../capsule_driver_usb_audio/src/uac/request.rs"]
mod request;
#[path = "../../../capsule_driver_usb_audio/src/uac/timing.rs"]
mod timing;

pub use alt::Stream;
pub use clock::clock;
pub use find::find;
pub use format::{RATE_HZ, SAMPLE_BYTES};
pub use request::{get_clock, select_first, set_clock, set_rate};
pub use timing::{timing, Timing};
