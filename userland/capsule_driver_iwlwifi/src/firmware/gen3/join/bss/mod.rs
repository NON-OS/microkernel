// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware's contexts for one access point, put up before the first
//! frame goes out and taken down in reverse, in the order Linux v6.12 runs
//! them for a station on the MLD API:
//!
//! 1. the PHY context on the channel, then its receive chains (RLC)
//!    (`iwl_mvm_phy_ctxt_add`);
//! 2. the link pointed at that PHY, then activated with the BSS's ACK rates
//!    (`__iwl_mvm_mld_assign_vif_chanctx`);
//! 3. the access point as station 0 (`iwl_mvm_mld_add_sta`), with `mfp` set
//!    until it is authorized;
//! 4. its management and data queues (`iwl_mvm_tvqm_enable_txq`), the ring
//!    starting at the write pointer each reply names;
//! 5. session protection, waited for until the firmware says it is on the
//!    channel (`iwl_mvm_mac_mgd_prepare_tx`; this driver waits, so the
//!    authentication frame is not queued for a radio still elsewhere).
//!
//! After association the MAC context is marked associated with the AID, the
//! station entry takes the AID, and the link the ERP preamble and slot of
//! the association response's capability (`iwl_mvm_mld_vif_cfg_changed_station`,
//! `BSS_CHANGED_ERP_SLOT`). Once the keys are in, the station entry takes
//! the negotiated `mfp` and session protection is cancelled. A context a
//! failed step left half up is taken down exactly as far as it went: every
//! step records itself in `stage` only once the firmware answered it.
//! Teardown is best effort (a firmware that stopped answering cannot be made
//! to), and flushes the station's queues before removing them, as
//! `iwl_mvm_mld_rm_sta` does.

mod after;
mod kinds;
mod link;
mod protect;
mod queues;
mod setup;
mod teardown;

pub use kinds::{Bss, SetupError, Stage, SESSION_WAIT_MS};
