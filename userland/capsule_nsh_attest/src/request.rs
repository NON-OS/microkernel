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

//! The enrolment request the machine makes for its operator, once its host
//! keys are kept and while it holds no enrolment certificate for them (none
//! at all, or one for keys it set aside after a new boot state). Three files
//! under /nonos/nsh, in the forms `nsh-enrol body` takes: `host.public`,
//! the host keys (Ed25519, then ML-DSA-65); `enrol.nonce`, the nonce in
//! hex; and `enrol.doc`, the kernel's NONOSATT document quoted over the
//! enrolment challenge for that nonce and those keys. So one image enrols
//! on any number of machines: each makes its own request, and the
//! operator signs a certificate for each with nsh-enrol, off the machine.

use crate::attest::quote;
use crate::challenge::{challenge, nonce_text};
use crate::keys::Held;
use crate::rng;
use crate::store::{self, Found};

pub const PUBLIC_PATH: &[u8] = b"/nonos/nsh/host.public";
pub const NONCE_PATH: &[u8] = b"/nonos/nsh/enrol.nonce";
pub const DOC_PATH: &[u8] = b"/nonos/nsh/enrol.doc";
/// The certificate the operator puts back (records.rs reads it).
pub const ENROL_PATH: &[u8] = b"/nonos/nsh/enrol";

/// What writing the request came to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Made {
    /// Written and kept.
    Written,
    /// The machine already holds its certificate; nothing to ask for.
    Enrolled,
    /// Not written now; asked again later.
    NotNow,
}

/// Whether `cert` is an enrolment certificate naming the host keys `held`.
/// Only the name is compared; the client checks the certificate whole.
fn names(cert: &[u8], held: &Held) -> bool {
    let public = held.public();
    let keys = nsh_crypto::key_digest(&public[..32], &public[32..]);
    nsh_wire::Enrol::parse(cert).is_ok_and(|e| e.host_keys == &keys)
}

/// Write the request for `held`, unless the machine is enrolled for it.
pub fn write(held: &Held) -> Made {
    match store::read(ENROL_PATH, nsh_front_wire::ENROL_MAX) {
        Found::Bytes(b) if names(&b, held) => return Made::Enrolled,
        Found::Fault => return Made::NotNow,
        _ => {}
    }
    let mut nonce = [0u8; 32];
    if !rng::fill(&mut nonce) {
        return Made::NotNow;
    }
    let Some(doc) = quote(&challenge(&nonce, held.public())) else {
        return Made::NotNow;
    };
    let written = store::put(PUBLIC_PATH, &held.public()[..])
        && store::put(NONCE_PATH, &nonce_text(&nonce))
        && store::put(DOC_PATH, &doc);
    if written {
        Made::Written
    } else {
        Made::NotNow
    }
}
