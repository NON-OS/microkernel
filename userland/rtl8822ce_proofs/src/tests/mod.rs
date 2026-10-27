// NONOS Operating System (AGPL-3.0-or-later)
//! The proofs, one module per subject; each rule is held by a test that a
//! mutant of it fails (verification/host_mutants/rtl8822ce.json).

mod bits_tests;
mod channel_tests;
mod coex_tests;
mod dack_codes_tests;
mod dack_iq_tests;
mod dack_measure_tests;
mod dack_readback_tests;
mod dack_run_tests;
mod efuse_facts_tests;
mod efuse_read_tests;
mod efuse_unpack_tests;
pub(crate) mod fixtures;
mod fw_tests;
mod ids_tests;
mod mac_ref;
mod mac_ref_wmac;
mod mac_tests;
mod pcie_tests;
mod power_tests;
mod pwr_tests;
mod rf_tests;
mod say_tests;
mod setup_tests;
mod signal_tests;
mod status_tests;
mod tables_tests;
mod trim_bias_tests;
mod trim_tests;
mod trx_mode_tests;
mod trx_tests;
mod txpower_tests;

use std::sync::{Mutex, MutexGuard};

/// One test at a time through the driver's statics (the RF path count, the
/// CCK gain bounds, the efuse stall record), which every thread shares.
pub fn lock() -> MutexGuard<'static, ()> {
    static ONE: Mutex<()> = Mutex::new(());
    ONE.lock().unwrap_or_else(|e| e.into_inner())
}
