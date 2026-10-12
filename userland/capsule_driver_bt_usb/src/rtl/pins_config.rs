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

//! The config files, from the same tag and licence as the patches. In
//! linux-firmware some config names are links to another file, so a chip
//! here names the file a link points to: rtl8723b_config.bin is
//! rtl8723bs_config.bin, rtl8723d's and rtl8821a's are rtl8821c's, and
//! rtl8761a's is rtl8761bu's.

use super::pin::{hex32, Pin};

pub const RTL8723BS_CONFIG: Pin = Pin {
    file: "rtl8723bs_config.bin",
    len: 64,
    sha256: hex32("a6319ce368257b45820fcf11f8821b6eea7ddcc649f59a282498bf7a33210103"),
};
pub const RTL8821C_CONFIG: Pin = Pin {
    file: "rtl8821c_config.bin",
    len: 10,
    sha256: hex32("292717b0610ce920f66b734c299d349ea59a4c4400044078d8fc65625f17e382"),
};
pub const RTL8761BU_CONFIG: Pin = Pin {
    file: "rtl8761bu_config.bin",
    len: 6,
    sha256: hex32("6c28a3f07c6a30ed208c4b64862a23f02b7d93543ea980edd24df16bab45095f"),
};
pub const RTL8822B_CONFIG: Pin = Pin {
    file: "rtl8822b_config.bin",
    len: 14,
    sha256: hex32("ea082136d5285655372e3805cd50d961bd7d42085c6fc1058a115a7e844df9c1"),
};
