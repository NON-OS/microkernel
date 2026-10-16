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

//! The NVM_GET_INFO reply (`iwl_nvm_get_info_rsp`, version 4 with
//! `IWL_UCODE_TLV_API_REGULATORY_NVM_INFO`, else version 3), read as Linux
//! v6.12 iwl-nvm-parse.c `iwl_get_nvm` does: the reply must be exactly its
//! version's size, the antennas come from the PHY SKU, LAR from the
//! regulatory block, and each channel profile entry's VALID bit says whether
//! the channel at that index (`iwl_uhb_nvm_channels` for a 6 GHz capable
//! part, `iwl_ext_nvm_channels` otherwise) is usable. With LAR the firmware
//! decides the regulatory domain and Linux keeps every channel (they "might
//! become valid later"); the scan here is passive, receive only, so it covers
//! them all then. Only 2.4 and 5 GHz channels are kept: this driver scans
//! those.

mod layout;
mod parse;

pub use layout::NVM_CHANNELS;
pub use parse::{parse, NvmInfo};
