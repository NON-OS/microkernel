// NONOS Operating System (AGPL-3.0-or-later)

//! Running a join to its end.

use super::drive::drive;
use super::radio::Radio;
use super::report::Report;
use nonos_wifi_core::mlme::{JoinRequest, Mlme};

/// Run a WPA2-PSK join (no SAE randomness) to completion, or until `budget`
/// receive passes finish without completing (each pass is one tick of the
/// clock here). `beacon` is the cached beacon for the target network, which
/// starts the machine; `snonce` is a fresh random nonce for the handshake.
#[cfg(test)]
pub fn run<R: Radio>(
    radio: &mut R,
    our_mac: [u8; 6],
    ssid: &[u8],
    passphrase: &[u8],
    beacon: &[u8],
    snonce: [u8; 32],
    budget: u32,
) -> Report {
    let mut pass = 0u64;
    let mut clock = || {
        pass += 1;
        pass
    };
    let mlme = Mlme::new(our_mac, ssid, passphrase, snonce);
    drive(radio, mlme, beacon, u64::from(budget), &mut clock)
}

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
