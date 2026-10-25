// NONOS Operating System (AGPL-3.0-or-later)

//! How far the contexts are up, why they could not be, and the contexts.

use super::super::super::station::session::Session;
use super::super::fw::CommandFailed;
use super::super::target::Target;

/// How long the firmware may take to put the radio on the channel.
pub const SESSION_WAIT_MS: u32 = 1000;

/// How far the contexts are up, each step only once the firmware answered.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Stage {
    Down,
    Phy,
    LinkActive,
    Station,
    MgmtQueue,
    DataQueue,
    OnChannel,
    Associated,
}

/// Why the contexts could not be put up.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetupError {
    Command(CommandFailed),
    /// A queue reply that was not the 8-byte structure.
    BadQueueReply,
    /// The firmware refused the session, or never put the radio on channel.
    NoSession(Option<Session>),
    /// The transmit region could not be cleared.
    Region,
}

impl From<CommandFailed> for SetupError {
    fn from(c: CommandFailed) -> Self {
        SetupError::Command(c)
    }
}

/// The contexts for one access point.
pub struct Bss {
    pub target: Target,
    /// The station's own address.
    pub addr: [u8; 6],
    /// The antennas the PHY configuration and the NVM both allow.
    pub tx_ant: u8,
    pub rx_ant: u8,
    pub stage: Stage,
    /// The association id, once associated.
    pub aid: u16,
    /// Session protection is running.
    pub session: bool,
}

impl Bss {
    pub fn new(target: Target, addr: [u8; 6], tx_ant: u8, rx_ant: u8) -> Self {
        Self { target, addr, tx_ant, rx_ant, stage: Stage::Down, aid: 0, session: false }
    }
}
