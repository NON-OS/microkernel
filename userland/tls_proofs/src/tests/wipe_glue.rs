// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Where the TLS secrets are wiped on paths the host cannot watch, read from
//! the source: the ECDHE secret, every HKDF block, the MAC's output, the
//! Finished key, a flight's private keys on drop and at each settle.

const SERVER_KEYS: &str = include_str!("../../../nonos_tls/src/server_keys.rs");
const P256: &str = include_str!("../../../nonos_tls/src/p256_share.rs");
const HKDF: &str = include_str!("../../../nonos_tls/src/hkdf.rs");
const HMAC: &str = include_str!("../../../nonos_tls/src/hmac_sha256.rs");
const FINISHED: &str = include_str!("../../../nonos_tls/src/finished_value.rs");
const VERIFY: &str = include_str!("../../../nonos_tls/src/finished_verify.rs");
const FLIGHT: &str = include_str!("../../../nonos_tls/src/flight.rs");
const SCHEDULE: &str = include_str!("../../../nonos_tls/src/schedule.rs");
const BROWSER: &str = include_str!("../../../capsule_browser/src/browser/fetch/types/tls_ctx.rs");
const WALLET: &str =
    include_str!("../../../capsule_wallet_nonos/src/wallet/net/step/exchange/send.rs");
const TERMINAL: &str = include_str!("../../../capsule_terminal/src/mixnet/exchange/tls_check.rs");

fn before(text: &str, a: &str, b: &str) -> bool {
    matches!((text.find(a), text.find(b)), (Some(x), Some(y)) if x < y)
}

#[test]
fn the_ecdhe_secret_goes_once_the_schedule_has_it() {
    let keys = "let keys = super::schedule::handshake_keys(&shared, &transcript.digest(), suite);";
    let after = &SERVER_KEYS[SERVER_KEYS.find(keys).expect("the schedule")..];
    assert!(before(after, keys, "super::traffic_keys::wipe(&mut shared);"));
    assert!(before(after, "super::traffic_keys::wipe(&mut shared);", "keys: keys?"));
    // A refused agreement wipes what it wrote; the agreement returns no copy.
    assert!(SERVER_KEYS.contains(
        "if !agree(client, &peer, &mut shared) {\n        super::traffic_keys::wipe(&mut shared);"
    ));
    assert!(SERVER_KEYS.contains("shared: &mut [u8; 32]) -> bool {"));
    assert!(P256.contains("out: &mut [u8; 32]) -> bool {"));
}

#[test]
fn hkdf_hmac_and_finished_wipe_what_they_derive() {
    assert!(HKDF.contains("wipe(&mut prev);\n    wipe(&mut block);\n    ok"));
    assert!(before(HMAC, "out.copy_from_slice(&bytes);", "super::traffic_keys::wipe(&mut bytes);"));
    assert!(FINISHED.contains("super::traffic_keys::wipe(&mut key);"));
    assert!(VERIFY.contains("super::traffic_keys::wipe(&mut key);"));
    assert!(SCHEDULE.contains("wipe(&mut early);") && SCHEDULE.contains("wipe(&mut derived);"));
}

#[test]
fn a_flight_wipes_on_drop_and_every_settle_wipes_both_keys() {
    assert!(FLIGHT.contains(
        "impl Drop for ClientFlight {\n    fn drop(&mut self) {\n        self.wipe_private();"
    ));
    for (name, text) in [("browser", BROWSER), ("wallet", WALLET), ("terminal", TERMINAL)] {
        assert!(text.contains(".wipe_private();"), "{name}");
        assert!(
            !text.contains("private = [0; 32]"),
            "{name}: a plain store the optimiser may drop"
        );
    }
}
