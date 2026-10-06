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

//! Telling the device the sizes this host takes, as Linux cdc_ncm_setup
//! does: the NTB input size (cdc_ncm_update_rxtx_max) and the largest
//! datagram (cdc_ncm_set_dgram_size).

use nonos_usbnet::xhci::BULK_MAX;
use nonos_usbnet::{Bus, Setup};

use super::function::NcmFunction;
use super::limits::rx_max;
use super::params::NtbParams;
use super::requests::{CAP_MAX_DATAGRAM_SIZE, CAP_NTB_INPUT_SIZE_8, GET_MAX_DATAGRAM_SIZE};
use super::requests::{SET_MAX_DATAGRAM_SIZE, SET_NTB_INPUT_SIZE};

/// CDC_NCM_MIN_DATAGRAM_SIZE and CDC_NCM_MAX_DATAGRAM_SIZE.
const DGRAM_MIN: u16 = 1514;
const DGRAM_MAX: u16 = 8192;

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
        }
    }
    max_datagram(bus, f);
    Ok(rx)
}

/// Only for a device that offers it; Linux reads the size first and
/// writes it only when it differs, and a failure changes nothing.
fn max_datagram<B: Bus>(bus: &mut B, f: &NcmFunction) {
    if f.caps & CAP_MAX_DATAGRAM_SIZE == 0 {
        return;
    }
    let want = f.max_segment.clamp(DGRAM_MIN, DGRAM_MAX);
    let mut now = [0u8; 2];
    let get = Setup::class(true, GET_MAX_DATAGRAM_SIZE, 0, f.comm);
    if bus.control_in(get, &mut now) != Ok(2) || u16::from_le_bytes(now) == want {
        return;
    }
    let set = Setup::class(false, SET_MAX_DATAGRAM_SIZE, 0, f.comm);
    let _ = bus.control_out(set, &want.to_le_bytes());
}
