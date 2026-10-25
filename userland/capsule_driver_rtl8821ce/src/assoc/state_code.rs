// NONOS Operating System (AGPL-3.0-or-later)

//! The state machine's progress as one byte.

use nonos_wifi_core::mlme::MlmeState;

/// The state machine's progress as a single byte, for reporting.
pub(super) fn state_code(s: MlmeState) -> u8 {
    match s {
        MlmeState::Scanning => 0,
        MlmeState::Authenticating => 1,
        MlmeState::Associating => 2,
        MlmeState::FourWay => 3,
        MlmeState::Connected => 4,
        MlmeState::Failed => 5,
    }
}
