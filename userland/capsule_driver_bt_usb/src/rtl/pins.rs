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

//! The patch files, from linux-firmware tag 20260916 (commit
//! ab23307cfe7f9366c819025ca3e4778299bc2db2), directory `rtl_bt/`, under
//! LICENSES/LICENCE.rtlwifi_firmware.txt: redistribution in binary form,
//! unmodified, with the notice kept; no reverse engineering. The owner
//! copies them into the store; the capsule only reads and checks them.

use super::pin::{hex32, Pin};

pub const RTL8723B_FW: Pin = Pin {
    file: "rtl8723b_fw.bin",
    len: 45_048,
    sha256: hex32("c9c8a9f6ebd90b7449a7e4d38ee39952dc7cc47d6c1d054e42cfbb5e4d818853"),
};
pub const RTL8723D_FW: Pin = Pin {
    file: "rtl8723d_fw.bin",
    len: 52_080,
    sha256: hex32("6b70723b2db9b658cd9535fe36f1494b4494699e1ad07920c9356f8bd90d9263"),
};
pub const RTL8821A_FW: Pin = Pin {
    file: "rtl8821a_fw.bin",
    len: 37_420,
    sha256: hex32("3ed5077f8f02ff2b06be257691dc72061efac976cd2dc174057833624c820b9d"),
};
pub const RTL8821C_FW: Pin = Pin {
    file: "rtl8821c_fw.bin",
    len: 54_636,
    sha256: hex32("dfe7ef772d078ec09490c02190a32fcd40b3eaf3b040049d8930eb80e6d5743c"),
};
pub const RTL8761A_FW: Pin = Pin {
    file: "rtl8761a_fw.bin",
    len: 74_488,
    sha256: hex32("581e5e81f5b51a29f475f8a276be95ad3210b3e3065987541c508e7f7a83a462"),
};
pub const RTL8761BU_FW: Pin = Pin {
    file: "rtl8761bu_fw.bin",
    len: 44_484,
    sha256: hex32("1d7a9597349ad89344fa16c1913d3e39e9a12e966e417ca16871bc79bbe59edb"),
};
pub const RTL8822B_FW: Pin = Pin {
    file: "rtl8822b_fw.bin",
    len: 51_176,
    sha256: hex32("eecef2c3b2ff03c3a41292a93a25eca6f4ec38eb4a7491a3651b530be89a04fc"),
};
