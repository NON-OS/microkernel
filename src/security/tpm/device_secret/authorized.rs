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

//! The policy the device secret's key is made under, in a session already
//! started: PolicyAuthorize over the approved PCR 9 by the release key, then
//! PolicyPCR over this machine's 0, 4 and 7 (mod.rs). The key it makes is
//! handed to `f` and flushed after, whatever `f` answers.

use super::approval::{Approval, POLICY_REF};
use super::command::{build_policy_authorize, parse_policy_authorize};
use super::consts::{APPROVED_PCRS, MACHINE_PCRS};
use super::public::p256_name;
use super::verify::verified;
use crate::security::tpm::machine_key::create::{build_create, parse_create};
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::policy::{
    build_get_digest, build_policy_pcr, parse_get_digest, parse_policy_pcr,
};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::KeyError;

pub(super) fn under_session<R>(
    session: u32,
    a: &Approval,
    f: impl FnOnce(u32, u32) -> Result<R, KeyError>,
) -> Result<R, KeyError> {
    parse_policy_pcr(&run(&build_policy_pcr(session, &APPROVED_PCRS))?)?;
    let approved = parse_get_digest(&run(&build_get_digest(session))?)?;
    let ticket = verified(a, &approved)?;
    let name = p256_name(&a.key_x, &a.key_y);
    parse_policy_authorize(&run(&build_policy_authorize(
        session, &approved, POLICY_REF, &name, &ticket,
    ))?)?;
    parse_policy_pcr(&run(&build_policy_pcr(session, &MACHINE_PCRS))?)?;
    let policy = parse_get_digest(&run(&build_get_digest(session))?)?;
    let key = parse_create(&run(&build_create(&policy))?)?;
    let result = f(key, session);
    let _ = run(&build_flush(key));
    result
}
