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

//! The loopback client's start: the user's config and grant, fresh
//! randomness from the system, and a connection to the host.

use super::loopback_host::{urandom, Clock};
use super::loopback_wire::Loop;
use crate::kit::host::HOST_NAME;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::K;
use crate::kit::user::{attester, policy, User};
use nsh_client::{Client, Config};
use nsh_crypto::Secret;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn dial(addr: SocketAddr, grant: &GrantB) -> (Loop, TcpStream) {
    let clock = Clock::start();
    let cfg = Config {
        host: HOST_NAME.to_vec(),
        now_ms: clock.now(),
        policy: policy(),
        grant: grant.bytes(),
    };
    let (mut seed, mut random) = ([0u8; 32], [0u8; 32]);
    assert!(urandom(&mut seed) && urandom(&mut random));
    let c = Client::new(cfg, attester(), User(K::User), Secret::take(&mut seed), random)
        .expect("start");
    let sock = TcpStream::connect(addr).expect("connect");
    sock.set_read_timeout(Some(Duration::from_millis(20))).expect("timeout");
    (c, sock)
}
