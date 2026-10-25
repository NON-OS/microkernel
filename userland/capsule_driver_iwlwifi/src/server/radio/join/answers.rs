// NONOS Operating System (AGPL-3.0-or-later)
//! The answers a connect gives without a join: a hidden network, which this
//! driver cannot find, and a network the hunt did not hear.

use crate::firmware::gen3::join::run::JoinEnd;
use crate::server::join_wire::{end_code, ConnectResult, CODE_NOT_FOUND};
use nonos_wifi_core::mlme::MlmeFailure;

use super::super::say::Line;

/// A hidden network: its beacons carry no name, and this driver sends no probe.
pub(super) fn hidden() -> ConnectResult {
    Line::new().text(b"join: a hidden network is not found by listening").send();
    ConnectResult::code(CODE_NOT_FOUND)
}
/// What a connect answers when the hunt heard `heard` beacons, none the network's.
pub(super) fn not_heard(heard: u32) -> ConnectResult {
    Line::new().text(b"join: network not heard among ").num(heard).text(b" beacons").send();
    let mut r = ConnectResult::code(CODE_NOT_FOUND);
    r.progress.recv = heard;
    r
}

/// What a connect answers when the network was heard, but only as one this
/// station cannot join as it claims to be, refused for `why`.
pub(super) fn refused(why: MlmeFailure) -> ConnectResult {
    Line::new().text(b"join: no access point of the network can be joined").send();
    let mut r = ConnectResult::code(0);
    (r.code, r.ap_code) = end_code(JoinEnd::Refused(why));
    r
}
