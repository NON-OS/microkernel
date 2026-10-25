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

//! The grant the client is given: only a base grant for the user's own
//! key is taken, so a grant the server would refuse is never sent.

use super::aborted_with;
use super::auth::answer;
use crate::kit::attest::TestAttester;
use crate::kit::keys;
use crate::kit::pair::config;
use crate::kit::server::Server;
use crate::kit::{grant, server_auth::SESSION_ID};
use nsh_client::{Client, Config, Refusal};
use nsh_crypto::Secret;
use nsh_wire::{AuthOk, Kind, Reason, WireError};

#[test]
fn a_broken_auth_ok_is_refused() {
    let mut zero_idle = vec![0u8; 64];
    let n = AuthOk::encode(&SESSION_ID, 600, &[b"IPC"], &mut zero_idle).unwrap();
    zero_idle[16..20].copy_from_slice(&0u32.to_le_bytes());
    let (r, mut p) = answer(Kind::AuthOk, &zero_idle[..n]);
    assert_eq!(r, Err(Refusal::Wire(WireError::Value)));
    aborted_with(&mut p, Reason::Protocol);
    let mut trailing = Server::auth_ok_body();
    trailing.push(0);
    assert_eq!(answer(Kind::AuthOk, &trailing).0, Err(Refusal::Wire(WireError::Trailing)));
    assert_eq!(answer(Kind::HostProof, &[0]).0, Err(Refusal::Wire(WireError::Order)));
    assert_eq!(answer(Kind::Output, b"x").0, Err(Refusal::Wire(WireError::Order)));
}

fn start(grant: Vec<u8>) -> Result<(), Refusal> {
    let cfg = Config { grant, ..config() };
    let c =
        Client::new(cfg, TestAttester::default(), keys::user_key(), Secret::new([1; 32]), [2; 32]);
    c.map(|_| ())
}

#[test]
fn only_a_base_grant_for_the_users_own_key_is_taken() {
    assert_eq!(start(grant::base()), Ok(()));
    assert_eq!(start(grant::grant(1, keys::stranger())), Err(Refusal::GrantSubject));
    assert_eq!(start(grant::escalation()), Err(Refusal::Unsendable(WireError::Value)));
    assert_eq!(start(b"NSHG".to_vec()), Err(Refusal::Unsendable(WireError::Short)));
    assert_eq!(start(Vec::new()), Err(Refusal::Unsendable(WireError::Short)));
}
