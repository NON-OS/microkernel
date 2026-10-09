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

//! The function's one-time setup while its data interface is in alternate
//! setting 0, as Linux cdc_ncm_init does it: the NTB parameters, CRC off,
//! and the 16-bit NTB format.

use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::{Bus, Setup};

use super::function::NcmFunction;
use super::params::{parse_params, NtbParams, NTB_PARAMETERS_LEN};
use super::requests::{CAP_CRC_MODE, CRC_NOT_APPENDED, GET_NTB_PARAMETERS};
use super::requests::{NTB16_FORMAT, SET_CRC_MODE, SET_NTB_FORMAT};

pub(super) fn init<B: Bus>(bus: &mut B, f: &NcmFunction) -> Result<NtbParams, (&'static str, i32)> {
    let mut raw = [0u8; NTB_PARAMETERS_LEN];
    let get = Setup::class(true, GET_NTB_PARAMETERS, 0, f.comm);
    let n = bus.control_in(get, &mut raw).map_err(|e| ("GET_NTB_PARAMETERS refused", e))?;
    let p = parse_params(&raw[..n]).ok_or(("NTB parameters short", E_IO))?;
    // Linux asks for datagrams without a CRC when the device could add one
    // and goes on if it is refused; an NDP with CRCs ("NCM1") is then
    // dropped on receive.
    if f.caps & CAP_CRC_MODE != 0 {
        let _ = bus.control_out(Setup::class(false, SET_CRC_MODE, CRC_NOT_APPENDED, f.comm), &[]);
    }
    // 16-bit NTBs are every device's default (NCM 1.0, 6.2.5); Linux sets
    // the format only on a device that offers 32-bit too, and a refusal
    // leaves it at 16.
    if p.ntb32() {
        let _ = bus.control_out(Setup::class(false, SET_NTB_FORMAT, NTB16_FORMAT, f.comm), &[]);
    }
    Ok(p)
}
