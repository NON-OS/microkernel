// NONOS Operating System (AGPL-3.0-or-later)

//! The host commands the bring-up sends after ALIVE, with the payloads Linux
//! v6.12 builds for this firmware (mvm/fw.c `iwl_run_unified_mvm_ucode` and
//! `iwl_mvm_up`, mvm/coex.c, fw/init.c, mvm/power.c, mvm/nvm.c, mvm/scan.c,
//! mvm/mld-mac.c, mvm/link.c). Each payload is a little-endian byte image of
//! its `struct iwl_*`; the sizes and field offsets are pinned in the proofs to
//! numbers compiled from the Linux headers. Legacy commands go out in
//! `LONG_GROUP`, as `iwl_trans_send_cmd` maps them with the wide header.

mod contexts;
mod ids;
mod init;
mod regulatory;

pub use contexts::*;
pub use ids::*;
pub use init::*;
pub use regulatory::*;
