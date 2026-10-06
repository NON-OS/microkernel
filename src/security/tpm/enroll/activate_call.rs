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

//! `activate_credential`, the five commands in order: the EK, a policy
//! session, PolicySecret, ActivateCredential, and both flushed whatever
//! happened. It holds the AK, the EK and one session at most.

use super::activate::{build_activate, check_challenge, parse_activate};
use super::activated::Activated;
use super::ek::build_create_ek;
use super::error::EnrollError;
use super::kind::EkKind;
use super::policy::{build_policy_secret, parse_policy_secret};
use crate::crypto::fill_random_bytes;
use crate::security::tpm::ak::load_ak;
use crate::security::tpm::machine_key::consts::NONCE_LEN;
use crate::security::tpm::machine_key::create::parse_create;
use crate::security::tpm::machine_key::derive::IN_PROGRESS;
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::session::{build_start, parse_start};

/// The secret in the registrar's challenge, unwrapped. `blob` and `secret`
/// are the bodies of its TPM2B_ID_OBJECT and TPM2B_ENCRYPTED_SECRET, bounded
/// before the TPM is asked anything.
pub fn activate_credential(
    kind: EkKind,
    blob: &[u8],
    secret: &[u8],
) -> Result<Activated, EnrollError> {
    check_challenge(blob, secret)?;
    let ak = load_ak()?;
    let _one = IN_PROGRESS.lock();
    let ek = parse_create(&run(&build_create_ek(kind))?)?;
    let result = in_session(ak, ek, blob, secret);
    let _ = run(&build_flush(ek));
    result
}

fn in_session(ak: u32, ek: u32, blob: &[u8], secret: &[u8]) -> Result<Activated, EnrollError> {
    let mut nonce = [0u8; NONCE_LEN];
    fill_random_bytes(&mut nonce);
    let session = parse_start(&run(&build_start(&nonce))?)?;
    let result = under_policy(ak, ek, session, blob, secret);
    let _ = run(&build_flush(session));
    result
}

fn under_policy(
    ak: u32,
    ek: u32,
    session: u32,
    blob: &[u8],
    secret: &[u8],
) -> Result<Activated, EnrollError> {
    parse_policy_secret(&run(&build_policy_secret(session))?)?;
    parse_activate(&run(&build_activate(ak, ek, session, blob, secret)?)?)
}
