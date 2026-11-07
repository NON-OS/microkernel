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

//! The server state machine. Each stage admits only its own frames, in the
//! one order docs/nsh/nsh-v1-wire.md allows; the keys are wiped at the end.

mod access;
mod admit;
mod api;
mod api_calls;
mod auth;
mod auth_end;
mod busy;
mod close;
mod dispatch;
mod end;
mod esc_answer;
mod escalate;
mod exit;
mod feed;
mod hello;
mod live;
mod log;
mod log_ckpt;
mod new;
mod proof;
mod record_parts;
mod send;
mod session;
mod stage;
mod start;
mod tick;
mod tick_session;

pub use busy::{busy, turned_away};
pub(crate) use live::Live;

use crate::{framer::Framer, Config, Event};
use alloc::vec::Vec;
use nsh_crypto::{Opener, Sealer, Secret, Transcript};
use nsh_roles::Login;
use stage::Stage;

pub struct Server<A, S, R> {
    cfg: Config,
    attest: A,
    signer: S,
    roles: R,
    stage: Stage,
    started_ms: u64,
    th: Transcript,
    th_hello: [u8; 64],
    doc_hash: [u8; 64],
    hs: Option<Secret<64>>,
    sealer: Option<Sealer>,
    opener: Option<Opener>,
    framer: Framer,
    out: Vec<u8>,
    events: Vec<Event>,
    admitted: Option<Login>, // the login the host was asked to start
    live: Option<Live>,
}
