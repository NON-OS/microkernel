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

//! The first phase, once the device is configured: the chip's settings
//! loaded, the PHY powered up, and the power saving that a driver which
//! polls its link and pipes has no use for turned off. Linux v6.12 does
//! this at probe; it then turns U1/U2, U2P3 and ALDPS back on, which this
//! driver does not, and it reads the burst size into one more setting
//! for the 5c20 and 5c30 chips, which this driver leaves as loaded.

mod b;
mod first;

use alloc::vec::Vec;

use super::model::Model;
use super::op::Op;
use super::usb_speed::UsbSpeed;

pub fn wake(m: Model, speed: UsbSpeed) -> Vec<Op> {
    match m {
        Model::Rtl8153(code) => first::wake(m, code, speed),
        Model::Rtl8153B(code) => b::wake(m, code),
    }
}
