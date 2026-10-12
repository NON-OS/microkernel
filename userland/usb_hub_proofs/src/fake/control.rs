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
//! The hub class requests a modelled hub answers.

use nonos_usb_hub::Setup;

use super::model::HubModel;

/// Answer `s` as `hub` would; Err(-32) (a STALL) for anything else.
pub fn answer(hub: &mut HubModel, s: Setup, data: &mut [u8]) -> Result<usize, i32> {
    let port = s.index as usize;
    let valid = (1..=hub.ports.len()).contains(&port);
    match (s.request_type, s.request, s.value) {
        (0xA0, 0x06, v) if v >> 8 == if hub.superspeed { 0x2A } else { 0x29 } => {
            let think = if hub.superspeed { 0 } else { hub.think << 5 };
            let d = [7, (v >> 8) as u8, hub.ports.len() as u8, think, 0, 50, 0];
            let n = d.len().min(data.len()).min(s.length as usize);
            data[..n].copy_from_slice(&d[..n]);
            Ok(n)
        }
        (0xA3, 0x00, 0) if valid => {
            let p = &mut hub.ports[port - 1];
            if p.resetting != 0 && p.resetting != u8::MAX {
                p.resetting -= 1;
                if p.resetting == 0 && p.device.is_some() {
                    (p.enabled, p.c_reset) = (true, true);
                }
            }
            data[..4].copy_from_slice(&hub.status(port));
            Ok(4)
        }
        (0x23, 0x03, 4) if valid => {
            let p = &mut hub.ports[port - 1];
            if p.resetting != u8::MAX {
                (p.resetting, p.enabled) = (2, false);
            }
            Ok(0)
        }
        (0x23, 0x03, 8) if valid => {
            hub.ports[port - 1].powered = true;
            Ok(0)
        }
        (0x23, 0x01, 16) if valid => {
            hub.ports[port - 1].c_connection = false;
            Ok(0)
        }
        (0x23, 0x01, 20) if valid => {
            hub.ports[port - 1].c_reset = false;
            Ok(0)
        }
        (0x20, 0x0C, _) if hub.superspeed => Ok(0),
        _ => Err(-32),
    }
}
