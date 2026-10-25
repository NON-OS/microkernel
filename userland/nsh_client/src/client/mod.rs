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

//! The client state machine. It moves through the handshake in the one
//! order nsh-v1-wire.md allows: `CLIENT_HELLO` out, `SERVER_HELLO` in,
//! `HOST_PROOF` in and checked, `USER_AUTH` out, `AUTH_OK` in, then the
//! session. Each stage admits only its own frames; anything else ends the
//! session, and the keys are dropped (and so wiped) when it ends.

mod access;
mod api;
mod api_close;
mod auth;
mod config;
mod end;
mod event;
mod feed;
mod grant;
mod hello;
mod new;
mod proof;
mod record;
mod send;
mod session;
mod stage;
mod user;

pub use config::Config;
pub use event::{Event, LoggedIn};

use crate::framer::Framer;
use crate::record::Bound;
use alloc::vec::Vec;
use nsh_crypto::{ClientKem, Opener, Sealer, Secret, Transcript};
use stage::Stage;

pub struct Client<A, S> {
    cfg: Config,
    attester: A,
    signer: S,
    stage: Stage,
    th: Transcript,
    th_hello: [u8; 64],
    /// What this session's record must name, filled in as it is learnt.
    bound: Bound,
    doc_hash: Option<[u8; 64]>,
    kem: Option<ClientKem>,
    hs: Option<Secret<64>>,
    sealer: Option<Sealer>,
    opener: Option<Opener>,
    framer: Framer,
    out: Vec<u8>,
    events: Vec<Event>,
    record: Vec<u8>,
}
