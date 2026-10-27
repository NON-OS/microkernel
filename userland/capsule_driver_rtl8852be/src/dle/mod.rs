// NONOS Operating System (AGPL-3.0-or-later)

//! The data link engine's memory plan and the host flow control over it.

mod hfc;
pub mod hfc_cfg;
mod init;
pub mod quota;

pub use hfc::hfc_init;
pub use init::{dle_init, DleFail};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfcFail {
    /// A channel's budget breaks the precommit or the public maximum.
    ChCfg(u8),
    /// The public groups do not add up to the public maximum.
    PubCfg,
    /// The public page accounting read back does not add up.
    PubInfo,
}
