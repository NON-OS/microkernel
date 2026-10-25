// NONOS Operating System (AGPL-3.0-or-later)

//! A join that reached an open port, and how one ends without.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::wpa::akm::Akm;

use super::super::bss::{Bss, SetupError};
use super::super::fw::CommandFailed;
use super::super::keys::{Installed, KeyError};
use super::super::link::Link;

/// How long a leaving station waits for its deauthentication to go out
/// before the queues are flushed.
pub const LEAVE_MS: u32 = 100;

/// A join that reached an open port.
pub struct Joined {
    pub bss: Bss,
    pub keys: Installed,
    pub link: Link,
    pub akm: Akm,
}

/// How a join ended without a port.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JoinEnd {
    /// The beacon is not the network's, or names no channel.
    NotTheNetwork,
    /// The MLME refused the network or the access point refused the station.
    Refused(MlmeFailure),
    /// The access point stopped answering.
    TimedOut,
    /// The firmware's contexts could not be put up.
    Setup(SetupError),
    /// A firmware command failed after the contexts were up, or the firmware
    /// raised its error cause.
    Firmware(Option<CommandFailed>),
    /// A key did not go in.
    Keys(KeyError),
}
