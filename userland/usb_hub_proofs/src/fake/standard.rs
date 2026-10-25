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
//! The standard requests a modelled device answers: its first eight
//! device descriptor bytes, its configuration (a hub's carries its class
//! 09h interface and status endpoint), and SET_CONFIGURATION.

use nonos_usb_hub::Setup;

use super::classes::HUB_CONFIG;
use super::model::{Device, Kind};

/// One vendor-specific interface, no endpoints.
const PLAIN: [u8; 18] = [9, 2, 18, 0, 1, 1, 0, 0x80, 50, 9, 4, 0, 0, 0, 0xFF, 0, 0, 0];

/// `dev`'s answer to a standard request, None when it is not one.
pub fn standard(dev: &Device, s: Setup, data: &mut [u8]) -> Option<Result<usize, i32>> {
    let mps0: u8 = if dev.speed == 2 { 8 } else { 64 };
    let config: &[u8] = if matches!(dev.kind, Kind::Hub(_)) { &HUB_CONFIG } else { &PLAIN };
    let bytes: &[u8] = match (s.request_type, s.request, s.value) {
        (0x80, 6, 0x0100) => &[18, 1, 0, 2, 0, 0, 0, mps0],
        (0x80, 6, 0x0200) => config,
        (0x00, 9, _) => &[],
        _ => return None,
    };
    let n = bytes.len().min(data.len()).min(s.length as usize);
    data[..n].copy_from_slice(&bytes[..n]);
    Some(Ok(n))
}
