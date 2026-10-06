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

//! The kernel's sequences with the AK passed in. Each test loads the AK into
//! its own TPM with the kernel's own command, where `load_ak` keeps one handle
//! for the process; `calls_tests` runs the kernel's entry points themselves.

use crate::security::tpm::ak::create_command;
use crate::security::tpm::enroll::activate::{build_activate, parse_activate};
use crate::security::tpm::enroll::ak_public::{build_read_public, parse_read_public};
use crate::security::tpm::enroll::ek::build_create_ek;
use crate::security::tpm::enroll::hash::{build_hash, parse_hash};
use crate::security::tpm::enroll::policy::{build_policy_secret, parse_policy_secret};
use crate::security::tpm::enroll::sign::{build_sign, parse_sign};
use crate::security::tpm::enroll::{Activated, EkKind, EnrollError, Public};
use crate::security::tpm::machine_key::create::parse_create;
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::session::{build_start, parse_start};

type Out<T> = Result<T, EnrollError>;

/// The kernel's AK in this test's TPM, and its public area as enroll reads it.
pub fn load_test_ak() -> (u32, Public) {
    let ak = parse_create(&run(&create_command()).expect("ak create")).expect("ak handle");
    let public = parse_read_public(&run(&build_read_public(ak)).expect("read")).expect("public");
    (ak, public)
}

/// `activate_credential`'s five commands, flushing as it does.
pub fn activate(ak: u32, kind: EkKind, blob: &[u8], secret: &[u8]) -> Out<Activated> {
    let ek = parse_create(&run(&build_create_ek(kind))?)?;
    let in_session = || -> Out<Activated> {
        let s = parse_start(&run(&build_start(&[0x42; 16]))?)?;
        let under = || -> Out<Activated> {
            parse_policy_secret(&run(&build_policy_secret(s))?)?;
            parse_activate(&run(&build_activate(ak, ek, s, blob, secret)?)?)
        };
        let r = under();
        let _ = run(&build_flush(s));
        r
    };
    let r = in_session();
    let _ = run(&build_flush(ek));
    r
}

/// `ak_sign`'s two commands; the digest TPM2_Hash made comes back too.
pub fn sign(ak: u32, msg: &[u8; 32]) -> Out<([u8; 32], [u8; 64])> {
    let (digest, ticket) = parse_hash(&run(&build_hash(msg))?)?;
    Ok((digest, parse_sign(&run(&build_sign(ak, &digest, &ticket))?)?))
}

/// The EK derived afresh, past the kernel's per-boot cache.
pub fn fresh_ek(kind: EkKind) -> Out<Public> {
    crate::security::tpm::enroll::ek_calls::derive_ek_public(kind)
}
