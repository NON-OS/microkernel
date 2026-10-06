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

//! Programming the part, one concern per file, in the order `run` calls.

mod cfg_done;
mod errata;
mod finish;
mod flush_rings;
mod flush_rx;
mod flush_tx;
mod hw_bits_82574;
mod hw_bits_pch;
mod interconnect;
mod k1;
mod lanphypc;
mod link;
mod mac_filter;
mod pch_prepare;
mod phy_reset;
mod post_phy_reset;
mod post_reset;
mod quiesce;
mod reset;
mod reset_block;
mod run;
mod rx_setup;
mod station_address;
mod tx_setup;
mod ulp;
mod ulp_host;

pub use finish::finish;
