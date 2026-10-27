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

use super::approval::Approval;
use super::authorized::under_session;
use super::consts::MACHINE_PCRS;
use super::draw::draw;
use crate::crypto::rng::fill_random_bytes_secure;
use crate::security::tpm::machine_key::consts::NONCE_LEN;
use crate::security::tpm::machine_key::derive::IN_PROGRESS;
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::session::{build_start, parse_start};
use crate::security::tpm::machine_key::KeyError;

/// The device secret on this machine, if this chain may have it: four field
/// words, each drawn by rejection.
pub fn device_secret(a: &Approval) -> Result<[u64; 4], KeyError> {
    with_approved_key(a, &MACHINE_PCRS, draw)
}

/// Run `f` with the key the approved policy over `machine` authorizes and
/// the session that satisfies it: the device secret's draw, or a kernel
/// label's key (approved_key.rs).
pub(super) fn with_approved_key<R>(
    a: &Approval,
    machine: &[u8],
    f: impl FnOnce(u32, u32) -> Result<R, KeyError>,
) -> Result<R, KeyError> {
    let _one = IN_PROGRESS.lock();
    let mut nonce = [0u8; NONCE_LEN];
    fill_random_bytes_secure(&mut nonce).map_err(|_| KeyError::NoRandomness)?;
    let session = parse_start(&run(&build_start(&nonce))?)?;
    let result = under_session(session, a, machine, f);
    let _ = run(&build_flush(session));
    result
}
