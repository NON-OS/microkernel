// NONOS Operating System (AGPL-3.0-or-later)
//! The proofs, one module per subject; all of them test-only, as lib.rs
//! mounts this file.

mod assoc_tests;
mod bounds_tests;
mod coex_tests;
mod connect_tests;
mod ddma_tests;
mod dma_tests;
mod download_tests;
mod efuse_tests;
mod fw_tests;
mod h2c_tests;
mod halt_tests;
mod link_tests;
pub(crate) mod linkport_tests;
mod mac_tests;
mod mac_trx_tests;
mod phy_rxpath_tests;
mod phy_tests;
mod prep_tests;
mod ps_tests;
mod pwr_cycle_tests;
mod pwr_switch_tests;
mod pwr_tests;
mod quiesce_tests;
mod rx_tests;
mod sae_forged_tests;
mod sae_join;
mod scan_tests;
mod sec_tests;
mod sections_tests;
mod serve_refuse_tests;
mod staging_tests;
mod status_tests;
mod tables_tests;
mod tx_clear_tests;
mod tx_tests;
mod wait_tests;
mod watch_tests;
