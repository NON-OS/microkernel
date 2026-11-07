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

//! Where net.nym's secrets are wiped on paths the host cannot drive, read
//! from the source: the derived key's inputs, the identity a request
//! installs, every request's bytes, and the link a lost gateway takes its key
//! with. The Gateway record copied into every session carries no key.

const DERIVE: &str = include_str!("../../capsule_net_nym/src/gateway_client/handshake/derive.rs");
const ESTABLISH: &str = include_str!("../../capsule_net_nym/src/gateway_client/establish.rs");
const SET_IDENTITY: &str =
    include_str!("../../capsule_net_nym/src/server/handlers/set_identity.rs");
const SERVE_ONE: &str = include_str!("../../capsule_net_nym/src/server/serve_one.rs");
const RUNNER: &str = include_str!("../../capsule_net_nym/src/server/runner.rs");
const LOST: &str = include_str!("../../capsule_net_nym/src/server/connect_tick.rs");
const GATEWAY: &str = include_str!("../../capsule_net_nym/src/state/gateway.rs");

pub(crate) fn before(text: &str, a: &str, b: &str) -> bool {
    matches!((text.find(a), text.find(b)), (Some(x), Some(y)) if x < y)
}

#[test]
fn the_derived_keys_inputs_and_the_registered_key_are_wiped() {
    assert!(before(DERIVE, "wipe(&mut dh);", "match derived {"));
    assert!(before(DERIVE, "wipe(&mut prk);", "match derived {"));
    assert!(DERIVE.contains("Err(e) => {\n            wipe(&mut key);"));
    assert!(before(ESTABLISH, "set_gateway_shared_key(gateway.link(), &key);", "wipe(&mut key);"));
}

#[test]
fn an_installed_identity_and_every_request_leave_no_copy() {
    assert!(before(SET_IDENTITY, "set_client_identity(&seed, &public);", "wipe(&mut seed);"));
    assert!(SERVE_ONE.trim_end().ends_with("crate::crypto::wipe::wipe(rx);\n}"));
    assert!(RUNNER.contains("super::serve_one::serve_one(sender, &mut rx[..n as usize], &mut tx);"));
    assert!(!RUNNER.contains("handlers::dispatch"), "every request goes through serve_one");
}

#[test]
fn a_lost_link_takes_its_key_and_no_gateway_record_carries_one() {
    let lost = &LOST[LOST.find("pub fn gateway_lost()").expect("present")..];
    assert!(lost[..lost.find("\n}").expect("ends")]
        .contains("crate::state::clear_gateway_shared_key();"));
    assert!(!GATEWAY.contains("shared_key: ["), "the Gateway record is copied everywhere");
}
