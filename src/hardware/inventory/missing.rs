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

use super::family::HardwareFamily::{self, *};

pub fn missing_path(family: HardwareFamily) -> Option<&'static str> {
    Some(match family {
        NetworkRealtekWifi => {
            "no driver for this Realtek Wi-Fi part (RTL8852AE, RTL8852CE and kin)"
        }
        DisplayNativeAmd | DisplayNativeNvidia => {
            "native display driver: firmware framebuffer only"
        }
        StorageVmd => "no driver of its own: the kernel brings up the drives behind it",
        _ => return None,
    })
}
