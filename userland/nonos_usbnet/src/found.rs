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

//! A device just addressed: its device descriptor and every configuration
//! it offers, read over endpoint 0, for the drivers to choose from.

use alloc::vec::Vec;

use crate::bus::Bus;
use crate::desc::{config_header, device_info, DeviceInfo, CONFIG, DEVICE};
use crate::setup::Setup;
use crate::xhci::{CONTROL_MAX, E_IO};

/// Configurations read past this are left: an iPhone offers five, and
/// none of the drivers here binds past the fourth.
const MAX_CONFIGS: u8 = 6;

pub struct Found {
    pub port: u8,
    pub info: DeviceInfo,
    /// Each configuration descriptor in index order, cut at CONTROL_MAX.
    pub configs: Vec<Vec<u8>>,
}

pub fn fetch<B: Bus>(bus: &mut B, port: u8) -> Result<Found, i32> {
    let mut dev = [0u8; 18];
    let n = bus.control_in(Setup::get_descriptor(DEVICE, 0), &mut dev)?;
    let info = device_info(&dev[..n]).ok_or(E_IO)?;
    let mut configs = Vec::new();
    for index in 0..info.configs.min(MAX_CONFIGS) {
        let mut head = [0u8; 9];
        let n = bus.control_in(Setup::get_descriptor(CONFIG, index), &mut head)?;
        let (total, _) = config_header(&head[..n]).ok_or(E_IO)?;
        let mut raw = alloc::vec![0u8; (total as usize).clamp(9, CONTROL_MAX)];
        let n = bus.control_in(Setup::get_descriptor(CONFIG, index), &mut raw)?;
        raw.truncate(n);
        configs.push(raw);
    }
    Ok(Found { port, info, configs })
}

/// A string descriptor's bytes, for a MAC address or a product name.
pub fn string<B: Bus>(bus: &mut B, index: u8, out: &mut [u8]) -> Result<usize, i32> {
    bus.control_in(Setup::get_descriptor(crate::desc::STRING, index), out)
}
