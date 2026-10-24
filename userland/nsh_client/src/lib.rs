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

//! The client side of nsh/1. A `Client` is sans-IO: the caller feeds it
//! the server's bytes and sends the bytes it hands back. It reads no clock
//! and no entropy: the time, the X-Wing seed, the client random and the
//! signing randomness all come from the caller. The host is accepted only
//! through its hybrid signature over the transcript, its enrolment under
//! the fleet authority, and an attestation check the caller supplies
//! (`Attester`); the user's key and grant leave only after all of these
//! pass. `verify_record` checks a session record offline. The
//! specification is docs/design/nsh-v1*.md.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
#[cfg(feature = "cli")]
extern crate std;

mod attest;
#[cfg(feature = "cli")]
pub mod cli;
mod client;
mod ct;
mod enrol_check;
mod error;
mod framer;
mod host_check;
mod host_error;
mod hybrid;
mod policy;
mod record;
mod signer;

pub use attest::{AttestRefusal, Attester, Quoted};
pub use client::{Client, Config, Event, LoggedIn};
pub use error::Refusal;
pub use host_error::{EnrolRefusal, HostRefusal};
pub use hybrid::Half;
pub use policy::{HybridKey, Policy};
pub use record::{verify_record, Bound, CkptRefusal, Mismatch, RecordRefusal, Registry};
pub use record::{Report, Verdict, CKPT_LEN};
pub use signer::{Rand, UserKey, UserSig, UserSigner};
