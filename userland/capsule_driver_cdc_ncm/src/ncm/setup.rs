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

//! Telling the device the sizes this host takes (NCM 1.0, 6.2.7 and 6.2.9):
//! the NTB input size and the largest datagram.

use nonos_usbnet::say::say;
use nonos_usbnet::xhci::BULK_MAX;
use nonos_usbnet::{Bus, Setup};

use super::datagram::max_datagram;
use super::function::NcmFunction;
use super::limits::rx_max;
use super::params::NtbParams;
use super::requests::{CAP_NTB_INPUT_SIZE_8, SET_NTB_INPUT_SIZE};

/// The NTB input size in force: the one asked for, or the device's own
/// when it refused and its own still fits one bulk transfer.
pub(super) fn setup<B: Bus>(
    bus: &mut B,
    f: &NcmFunction,
    p: &NtbParams,
) -> Result<usize, (&'static str, i32)> {
    let mut rx = rx_max(p.in_max);
    if rx != p.in_max as usize {
        let size = (rx as u32).to_le_bytes();
        // NCM 1.0, 6.2.7: the 8-byte form adds wNtbInMaxDatagrams, 0 for
        // no limit, and a reserved word.
        let long = [size[0], size[1], size[2], size[3], 0, 0, 0, 0];
        let data: &[u8] = if f.caps & CAP_NTB_INPUT_SIZE_8 != 0 { &long } else { &size };
        let set = Setup::class(false, SET_NTB_INPUT_SIZE, 0, f.comm);
        if let Err(e) = bus.control_out(set, data) {
            if p.in_max as usize > BULK_MAX {
                return Err(("SET_NTB_INPUT_SIZE refused, blocks too large", e));
            }
            rx = p.in_max as usize;
            // Bounded and safe, but under the 2048 the request allows: said
            // once, at bind, and the device kept.
            if rx < 2048 {
                say(b"cdc-ncm", &[b"input size refused; the device's own, under 2048, kept"]);
            }
        }
    }
    max_datagram(bus, f);
    Ok(rx)
}
