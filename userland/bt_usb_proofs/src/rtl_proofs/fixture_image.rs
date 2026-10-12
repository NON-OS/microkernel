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

//! A patch file with two patches, a config, and a store holding them,
//! for the image and setup proofs.

use super::fixture::{epatch, Mem, VERSION};

pub const PATCH: &[u8] = &[0x11, 0x22, 0x33, 0x44, 0x55, 0, 0, 0, 0];
pub const CONFIG: &[u8] = &[0x55, 0xab, 0x23, 0x87, 0x04, 0x00, 0xd9, 0x00, 0x01, 0x0f];
pub const FW: &str = "rtl8821c_fw.bin";
pub const CFG: &str = "rtl8821c_config.bin";

pub fn store(fw: &[u8], cfg: Option<&[u8]>) -> Mem {
    let mut m = Mem::default();
    m.files.insert(FW, fw.to_vec());
    if let Some(c) = cfg {
        m.files.insert(CFG, c.to_vec());
    }
    m
}

pub fn file(project: u8) -> Vec<u8> {
    epatch(VERSION, &[(1, &[9; 8]), (2, PATCH)], project)
}
