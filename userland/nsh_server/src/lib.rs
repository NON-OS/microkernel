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

//! The host side of nsh/1. A `Server` is sans-IO: the caller feeds it the
//! client's bytes and sends the bytes it hands back. It reads no clock and
//! no entropy: the time and every random byte are the caller's. The host
//! proves itself with its hybrid keys over the transcript, its enrolment
//! certificate and the kernel's attestation document for this handshake's
//! challenge; a user is admitted only on a base grant the host's role
//! table allows, checked in the order docs/nsh/nsh-v1-roles.md gives. Every
//! session is written to a hash-chained record the host keys sign. The
//! specification is docs/nsh/nsh-v1*.md.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

mod admission;
mod call_error;
mod config;
mod end_reason;
mod event;
mod framer;
mod host;
mod host_key;
mod record;
mod role_store;
mod roles;
mod server;
mod why;

pub use admission::Admission;
pub use call_error::CallError;
pub use config::{BadConfig, Config, MIN_RECORD};
pub use end_reason::{EndReason, EscEnd};
pub use event::Event;
pub use host::{AttestFault, Attestation, HostSig, HostSigner, Rand};
pub use host_key::HostKey;
pub use record::{CKPT_MAGIC, HEADER_MAX as RECORD_HEADER_MAX};
pub use role_store::RoleStore;
pub use roles::Roles;
pub use server::{busy, turned_away, Server};
pub use why::Why;
