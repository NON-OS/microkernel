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

//! Devices whose blocks are not padded to the full NTB size: Linux
//! cdc_devs[] gives DisplayLink docking stations (vendor 0x17e9)
//! cdc_ncm_zlp_info, FLAG_SEND_ZLP, which skips that padding in
//! cdc_ncm_fill_tx_frame.

const SEND_ZLP_VENDORS: &[u16] = &[0x17e9];

pub fn sends_zlp(vendor: u16) -> bool {
    SEND_ZLP_VENDORS.contains(&vendor)
}
