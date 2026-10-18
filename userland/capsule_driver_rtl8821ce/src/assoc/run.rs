// NONOS Operating System (AGPL-3.0-or-later)

//! Running a join to its end.

use super::drive::drive;
use super::radio::Radio;
use super::report::Report;
use nonos_wifi_core::mlme::{JoinRequest, Mlme};

/// Run a join under the request's policy (SAE when the network offers it,
/// PSK otherwise, never PSK for a network saved as WPA3) to completion, or
/// until `budget_ms` milliseconds of `now_ms` pass without completing.
/// `now_ms` is read once per receive pass.
pub fn run_join<R: Radio>(
    radio: &mut R,
    req: &JoinRequest,
    beacon: &[u8],
    budget_ms: u64,
    now_ms: &mut dyn FnMut() -> u64,
) -> Report {
    drive(radio, Mlme::join(req), beacon, budget_ms, now_ms)
}
