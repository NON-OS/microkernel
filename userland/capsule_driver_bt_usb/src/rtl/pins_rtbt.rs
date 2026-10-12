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

//! The second-layout patch files, from the same linux-firmware tag and
//! commit, directory and licence as `pins.rs`. None of these chips has a
//! USB config in linux-firmware, so none is pinned. RTL8852C's file is the
//! `_v2` one, which Linux's driver also asks for first.

use super::pin::{hex32, Pin};

pub const RTL8822CU_FW: Pin = Pin {
    file: "rtl8822cu_fw.bin",
    len: 37_327,
    sha256: hex32("bb8b1221b62749baebfa3e60271d6d90b1db5e3a480e3255be82c9fb58fe9577"),
};
pub const RTL8852AU_FW: Pin = Pin {
    file: "rtl8852au_fw.bin",
    len: 63_352,
    sha256: hex32("fb52f72e986d4c96dbf7263650d3766abcbcf10c0a4f23a065ba2c388b19c125"),
};
pub const RTL8852BU_FW: Pin = Pin {
    file: "rtl8852bu_fw.bin",
    len: 129_733,
    sha256: hex32("c2b0d34a4ff8a8ea136993f0dc4ba657522d84231b4094c2a53cda39afc91c57"),
};
pub const RTL8852BTU_FW: Pin = Pin {
    file: "rtl8852btu_fw.bin",
    len: 128_140,
    sha256: hex32("bd162fd3b70e805cf7a8098f22f4ddf5d89da58fafff97d87da1a1e2e0cbeb80"),
};
pub const RTL8852CU_FW: Pin = Pin {
    file: "rtl8852cu_fw_v2.bin",
    len: 112_953,
    sha256: hex32("d1314df53b8f4282f9db57a727ba77c180bc6fd05bf2c306e960326a4ef48660"),
};
pub const RTL8851BU_FW: Pin = Pin {
    file: "rtl8851bu_fw.bin",
    len: 49_760,
    sha256: hex32("7fcf88c00b2548a5ff157102a35722db5e4be655dab1ce8d9bddb6df85f88f24"),
};
