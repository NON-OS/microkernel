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
//! step at a time as well as the whole sequence.

#[path = "../../../capsule_driver_e1000e/src/init/cfg_done.rs"]
pub mod cfg_done;
#[path = "../../../capsule_driver_e1000e/src/init/errata.rs"]
pub mod errata;
#[path = "../../../capsule_driver_e1000e/src/init/finish.rs"]
pub mod finish;
#[path = "../../../capsule_driver_e1000e/src/init/flush_rings.rs"]
pub mod flush_rings;
#[path = "../../../capsule_driver_e1000e/src/init/flush_rx.rs"]
pub mod flush_rx;
#[path = "../../../capsule_driver_e1000e/src/init/flush_tx.rs"]
pub mod flush_tx;
#[path = "../../../capsule_driver_e1000e/src/init/hw_bits_82574.rs"]
pub mod hw_bits_82574;
#[path = "../../../capsule_driver_e1000e/src/init/hw_bits_pch.rs"]
pub mod hw_bits_pch;
#[path = "../../../capsule_driver_e1000e/src/init/interconnect.rs"]
pub mod interconnect;
#[path = "../../../capsule_driver_e1000e/src/init/k1.rs"]
pub mod k1;
#[path = "../../../capsule_driver_e1000e/src/init/lanphypc.rs"]
pub mod lanphypc;
#[path = "../../../capsule_driver_e1000e/src/init/link.rs"]
pub mod link;
#[path = "../../../capsule_driver_e1000e/src/init/mac_filter.rs"]
pub mod mac_filter;
#[path = "../../../capsule_driver_e1000e/src/init/pch_prepare.rs"]
pub mod pch_prepare;
#[path = "../../../capsule_driver_e1000e/src/init/phy_reset.rs"]
pub mod phy_reset;
#[path = "../../../capsule_driver_e1000e/src/init/post_phy_reset.rs"]
pub mod post_phy_reset;
#[path = "../../../capsule_driver_e1000e/src/init/post_reset.rs"]
pub mod post_reset;
#[path = "../../../capsule_driver_e1000e/src/init/quiesce.rs"]
pub mod quiesce;
#[path = "../../../capsule_driver_e1000e/src/init/reset.rs"]
pub mod reset;
#[path = "../../../capsule_driver_e1000e/src/init/reset_block.rs"]
pub mod reset_block;
#[path = "../../../capsule_driver_e1000e/src/init/run.rs"]
pub mod run;
#[path = "../../../capsule_driver_e1000e/src/init/rx_setup.rs"]
pub mod rx_setup;
#[path = "../../../capsule_driver_e1000e/src/init/station_address.rs"]
pub mod station_address;
#[path = "../../../capsule_driver_e1000e/src/init/tx_setup.rs"]
pub mod tx_setup;
#[path = "../../../capsule_driver_e1000e/src/init/ulp.rs"]
pub mod ulp;
#[path = "../../../capsule_driver_e1000e/src/init/ulp_host.rs"]
pub mod ulp_host;
