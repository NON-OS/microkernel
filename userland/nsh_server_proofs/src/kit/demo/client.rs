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

//! The demo client over a real TCP connection: the real `nsh_client` with the
//! real `NonosAttester`, the fixture user and the fixture client policy. It
//! checks the host's attestation before it sends the user credential, exactly
//! as the shipping `nsh` binary does; nothing here weakens that check.

use super::{urandom, Clock};
use crate::kit::host::HOST_NAME;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::K;
use crate::kit::user::{attester, policy, User};
use nsh_client::{Client, Config, Event as C, NonosAttester, Refusal};
use nsh_crypto::Secret;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

pub type Loop = Client<NonosAttester, User>;

/// Shell output, whether the client logged in, any refusal, and clean exit.
#[derive(Default)]
pub struct Outcome {
    pub output: Vec<u8>,
    pub logged_in: bool,
    pub refusal: Option<Refusal>,
    pub clean_exit: bool,
}

/// Verify the host over `sock`, log in with the fixture grant, type each of
/// `lines` once logged in, and run until the session ends.
pub fn run_client(mut sock: TcpStream, lines: &[&[u8]]) -> Outcome {
    sock.set_read_timeout(Some(Duration::from_millis(20))).ok();
    let clock = Clock::start();
    let cfg = Config { host: HOST_NAME.to_vec(), now_ms: clock.now(), policy: policy(), grant: GrantB::default().bytes() };
    let (mut seed, mut random) = ([0u8; 32], [0u8; 32]);
    assert!(urandom(&mut seed) && urandom(&mut random));
    let mut c: Loop = Client::new(cfg, attester(), User(K::User), Secret::take(&mut seed), random).expect("start");
    let (mut out, mut typed, mut buf) = (Outcome::default(), false, vec![0u8; 65_536]);
    let deadline = Instant::now() + Duration::from_secs(120);
    while !c.is_over() && Instant::now() < deadline {
        sock.write_all(&c.take_output()).ok();
        match sock.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => out.refusal = c.feed(&buf[..n], &mut urandom).err().or(out.refusal.take()),
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(_) => break,
        }
        for e in c.events() {
            match &e {
                C::LoggedIn(_) => out.logged_in = true,
                C::Output(b) => out.output.extend_from_slice(b),
                C::Exit(0) => out.clean_exit = true,
                _ => {}
            }
        }
        if out.logged_in && !typed {
            lines.iter().try_for_each(|l| c.input(l)).expect("input");
            typed = true;
        }
    }
    sock.write_all(&c.take_output()).ok();
    out
}
