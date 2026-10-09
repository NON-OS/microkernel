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

//! The 802.11 frame layer the association MLME and the data path are made
//! of: the MAC header, the management frames a scan, an authentication
//! (open or SAE) and an association exchange are built from and parsed out
//! of, the beacon elements a join needs, and the data frame with its CCMP
//! protection. It holds no device state and touches no register, so it is
//! proven on the host in `nonos_wifi_core_proofs` and `rtl8821ce_proofs`.

pub mod auth;
pub mod ccmp;
pub mod data;
pub mod header;
pub mod ies;
pub mod mgmt;
pub mod parse;
