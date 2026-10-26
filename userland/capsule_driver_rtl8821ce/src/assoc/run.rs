// NONOS Operating System (AGPL-3.0-or-later)

//! Running a join to its end.

use super::clock::JoinClock;
use super::drive::drive;
use super::radio::Radio;
use super::report::Report;
use nonos_wifi_core::mlme::{JoinRequest, Mlme};

/// Run a join under the request's policy (SAE when the network offers it,
/// PSK otherwise, never PSK for a network saved as WPA3) to completion, or
/// until `budget_ms` milliseconds of `clock` pass without completing. The
/// clock is read once per receive pass, and idles each pass that heard nothing.
pub fn run_join<R: Radio>(
    radio: &mut R,
    req: &JoinRequest,
    beacon: &[u8],
    budget_ms: u64,
    clock: &mut dyn JoinClock,
) -> Report {
    drive(radio, Mlme::join(req), beacon, budget_ms, clock)
}
