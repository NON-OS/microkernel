// NONOS Operating System (AGPL-3.0-or-later)

//! The groups and opcodes of the join's commands (fw/api/commands.h,
//! mac-cfg.h, datapath.h) and the version of each one's layout this driver
//! encodes. A firmware file says, per command, which version it runs
//! (`IWL_UCODE_TLV_FW_CMD_VERSIONS`); a command it does not list runs the
//! version Linux falls back to. [`check`] holds the loaded firmware to this
//! table before anything is sent, so a firmware that moved a layout is
//! refused instead of handed bytes it reads differently. The versions are
//! the ones both bundled SO images report.

mod opcodes;
mod versions;

pub use opcodes::*;
pub use versions::*;
