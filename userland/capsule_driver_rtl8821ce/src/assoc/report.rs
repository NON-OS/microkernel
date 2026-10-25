// NONOS Operating System (AGPL-3.0-or-later)

//! What a join reports, ended or not.

use super::outcome::Outcome;
use super::radio::Counters;
use nonos_wifi_core::mlme::{Mlme, MlmeFailure, MlmeState};
use nonos_wifi_core::wpa::akm::Akm;

/// The join outcome together with how far it got: frames transmitted, frames
/// received, and the state the machine reached. The counts turn a bare timeout
/// into a diagnosis, one frame sent and none received points at transmit, several
/// exchanged points past it.
pub struct Report {
    pub outcome: Outcome,
    pub sent: u32,
    pub recv: u32,
    /// Data frames delivered off the ring. Distinguishes "the card never gave us a
    /// data frame" (a receive-filter or link-state problem) from "data frames
    /// arrive but none parse as EAPOL" (a framing problem).
    pub data: u32,
    /// EAPOL key frames recognized off the air. Zero while `state` is FourWay with
    /// data frames arriving means the handshake frame is not being parsed as EAPOL;
    /// zero with no data frames means it never reached us.
    pub eapol: u32,
    /// The first non-EAPOL data frame's frame-control (low 16) and length (high 16).
    pub probe: u32,
    /// Deauthenticate/disassociate frames received. Non-zero means the access
    /// point started the connection then tore it down, which it does after it
    /// sends the first EAPOL frame and gets no reply: proof the AP is trying to
    /// key us and the handshake frame is being lost on our side.
    pub deauth: u32,
    /// Data frames whose receiver address is our own MAC (unicast to us) rather
    /// than a group address. The EAPOL handshake arrives this way, so zero here
    /// while broadcast data arrives means unicast data is being dropped, not that
    /// the AP is silent.
    pub to_us: u32,
    pub state: u8,
    /// Why the machine failed, when it did.
    pub failure: Option<MlmeFailure>,
    /// The AKM chosen from the beacon, once one was.
    pub akm: Option<Akm>,
}

/// The state machine's progress as a single byte, for reporting.
fn state_code(s: MlmeState) -> u8 {
    match s {
        MlmeState::Scanning => 0,
        MlmeState::Authenticating => 1,
        MlmeState::Associating => 2,
        MlmeState::FourWay => 3,
        MlmeState::Connected => 4,
        MlmeState::Failed => 5,
    }
}

// Wrap an outcome with the progress counters and the final state.
pub(super) fn report(mlme: &Mlme, outcome: Outcome, c: Counters) -> Report {
    Report {
        outcome,
        sent: c.sent,
        recv: c.recv,
        data: c.data,
        eapol: c.eapol,
        probe: c.probe,
        deauth: c.deauth,
        to_us: c.to_us,
        state: state_code(mlme.state()),
        failure: mlme.failure(),
        akm: mlme.akm(),
    }
}
