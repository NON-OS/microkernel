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

//! The driver's bring-up files, each public here so a test can drive one
//! step at a time as well as the whole sequence. They are mounted from the
//! driver's init directory as they are, siblings of one another, as the
//! driver's `super::` paths need.

#[path = "../../../capsule_driver_e1000e/src/init"]
mod files {
    pub mod cfg_done;
    pub mod errata;
    pub mod finish;
    pub mod flush_rings;
    pub mod flush_rx;
    pub mod flush_tx;
    pub mod hw_bits_82574;
    pub mod hw_bits_pch;
    pub mod interconnect;
    pub mod k1;
    pub mod lanphypc;
    pub mod link;
    pub mod mac_filter;
    pub mod own_rings;
    pub mod pch_prepare;
    pub mod phy_reset;
    pub mod post_phy_reset;
    pub mod post_reset;
    pub mod quiesce;
    pub mod recover;
    pub mod reset;
    pub mod reset_block;
    pub mod reset_part;
    pub mod run;
    pub mod rx_setup;
    pub mod start;
    pub mod station_address;
    pub mod tx_setup;
    pub mod ulp;
    pub mod ulp_host;
}

pub use files::*;
pub use quiesce::{run as quiesce, stop_dma, stop_units};
pub use recover::recover;
