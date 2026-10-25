// NONOS Operating System (AGPL-3.0-or-later)
//! The proofs of the gen3 firmware: its image, layout, boot and radio, with
//! the device model they run against.

#[cfg(test)]
mod dram_map_tests;
#[cfg(test)]
mod gen3_armed;
#[cfg(test)]
mod gen3_boot_tests;
#[cfg(test)]
mod gen3_booted;
#[cfg(test)]
mod gen3_cmdq_tests;
#[cfg(test)]
mod gen3_fetch_tests;
#[cfg(test)]
mod gen3_image_tests;
#[cfg(test)]
mod gen3_layout_tests;
#[cfg(test)]
mod gen3_master_tests;
#[cfg(test)]
pub(crate) mod gen3_model;
#[cfg(test)]
mod gen3_parse_tests;
#[cfg(test)]
mod gen3_pnvm_tests;
#[cfg(test)]
mod gen3_radio_tests;
#[cfg(test)]
mod gen3_restart_tests;
#[cfg(test)]
mod gen3_stop_tests;
#[cfg(test)]
mod gen3_sweep_uid_tests;
#[cfg(test)]
mod gen3_tests;
#[cfg(test)]
mod prph_scratch_tests;
#[cfg(test)]
mod restart_policy_tests;
#[cfg(test)]
mod station_cmd_tests;
#[cfg(test)]
mod tx_path_tests;
#[cfg(test)]
mod tx_stuck_tests;
