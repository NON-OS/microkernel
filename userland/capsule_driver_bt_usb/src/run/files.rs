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

//! The Realtek files, built into the capsule so its seal covers them and
//! it needs no filesystem authority. The owner adds them to the tree under
//! `nonos-bootloader/firmware/rtl_bt/` (Decision 19); `build.rs` refuses a
//! build whose files do not match their pins, and `rtl` checks them again
//! before each download.

use alloc::vec::Vec;

use crate::rtl::Blobs;

macro_rules! rtl_bt {
    ($($name:literal),* $(,)?) => {
        &[$(($name, include_bytes!(concat!("../../../../nonos-bootloader/firmware/rtl_bt/", $name)))),*]
    };
}

const FILES: &[(&str, &[u8])] = rtl_bt![
    "rtl8723b_fw.bin",
    "rtl8723d_fw.bin",
    "rtl8821a_fw.bin",
    "rtl8821c_fw.bin",
    "rtl8761a_fw.bin",
    "rtl8761bu_fw.bin",
    "rtl8822b_fw.bin",
    "rtl8723bs_config.bin",
    "rtl8821c_config.bin",
    "rtl8761bu_config.bin",
    "rtl8822b_config.bin",
    "rtl8822cu_fw.bin",
    "rtl8852au_fw.bin",
    "rtl8852bu_fw.bin",
    "rtl8852btu_fw.bin",
    "rtl8852cu_fw_v2.bin",
    "rtl8851bu_fw.bin",
];

pub struct Files;

impl Blobs for Files {
    fn read(&mut self, file: &str, max: usize) -> Option<Vec<u8>> {
        let (_, bytes) = FILES.iter().find(|(name, _)| *name == file)?;
        Some(bytes[..bytes.len().min(max)].to_vec())
    }
}
