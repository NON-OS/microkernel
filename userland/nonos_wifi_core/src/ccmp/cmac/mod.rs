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

//! AES-128-CMAC (RFC 4493). The EAPOL-Key MIC for the SAE and PSK-SHA256
//! AKMs is AES-128-CMAC under the KCK (IEEE Std 802.11-2020, 12.7.3, key
//! descriptor versions 0 and 3), and BIP-CMAC-128 signs with it. The MAC
//! itself is the system's one, in nonos_aes.

mod single;

pub use nonos_aes::{aes_cmac as aes_cmac_parts, CMAC_BYTES as CMAC_LEN};
pub use single::aes_cmac;
