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

//! The derivation, in one policy session, with every slot given back on every
//! path. The external key is flushed before the primary is created, so the
//! sequence never holds more than two slots beside the attestation key's.

use super::approval::{Approval, POLICY_REF};
use super::command::{build_policy_authorize, parse_policy_authorize};
use super::consts::{APPROVED_PCRS, MACHINE_PCRS};
use super::draw::draw;
use super::public::p256_name;
use super::verify::verified;
use crate::crypto::fill_random_bytes;
use crate::security::tpm::machine_key::consts::NONCE_LEN;
use crate::security::tpm::machine_key::create::{build_create, parse_create};
use crate::security::tpm::machine_key::derive::IN_PROGRESS;
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::policy::{build_get_digest, build_policy_pcr, parse_get_digest, parse_policy_pcr};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::session::{build_start, parse_start};
use crate::security::tpm::machine_key::KeyError;

/// The device secret on this machine, if this chain may have it: four field
/// words, each drawn by rejection.
pub fn device_secret(a: &Approval) -> Result<[u64; 4], KeyError> {
    let _one = IN_PROGRESS.lock();
    let mut nonce = [0u8; NONCE_LEN];
    fill_random_bytes(&mut nonce);
    let session = parse_start(&run(&build_start(&nonce))?)?;
    let result = under_session(session, a);
    let _ = run(&build_flush(session));
    result
}

fn under_session(session: u32, a: &Approval) -> Result<[u64; 4], KeyError> {
    parse_policy_pcr(&run(&build_policy_pcr(session, &APPROVED_PCRS))?)?;
    let approved = parse_get_digest(&run(&build_get_digest(session))?)?;
    let ticket = verified(a, &approved)?;
    let name = p256_name(&a.key_x, &a.key_y);
    parse_policy_authorize(&run(&build_policy_authorize(session, &approved, POLICY_REF, &name, &ticket))?)?;
    parse_policy_pcr(&run(&build_policy_pcr(session, &MACHINE_PCRS))?)?;
    let policy = parse_get_digest(&run(&build_get_digest(session))?)?;
    let key = parse_create(&run(&build_create(&policy))?)?;
    let result = draw(key, session);
    let _ = run(&build_flush(key));
    result
}
