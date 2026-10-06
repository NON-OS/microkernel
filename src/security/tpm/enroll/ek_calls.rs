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

//! The EK's public area as the kernel reads it: under the lock the machine
//! key's derivations take, since they share the transient slots, and with the
//! EK flushed on every path.

use super::ek::{build_create_ek, parse_ek};
use super::error::EnrollError;
use super::kind::EkKind;
use super::public::Public;
use spin::Mutex;
use crate::security::tpm::machine_key::create::parse_create;
use crate::security::tpm::machine_key::derive::IN_PROGRESS;
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::run::run;

/// Each EK's public area, once derived this boot.
static DERIVED: Mutex<[Option<Public>; 2]> = Mutex::new([None, None]);

/// The EK's public area and name, for the registrar to match against the EK
/// certificate and to make its credential to. Derived once a boot: the EK is
/// a primary under the endorsement seed with a fixed template, so a second
/// derivation is the same key, and an RSA one holds the TPM for seconds.
pub fn ek_public(kind: EkKind) -> Result<Public, EnrollError> {
    let slot = match kind {
        EkKind::Rsa2048 => 0,
        EkKind::EccP256 => 1,
    };
    if let Some(p) = DERIVED.lock()[slot].clone() {
        return Ok(p);
    }
    let p = derive_ek_public(kind)?;
    DERIVED.lock()[slot] = Some(p.clone());
    Ok(p)
}

/// The derivation itself, every time.
pub(in crate::security::tpm) fn derive_ek_public(kind: EkKind) -> Result<Public, EnrollError> {
    let _one = IN_PROGRESS.lock();
    let resp = run(&build_create_ek(kind))?;
    let ek = parse_create(&resp)?;
    let public = parse_ek(&resp, kind);
    let _ = run(&build_flush(ek));
    public
}
