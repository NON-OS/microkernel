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

//! The certificate's body, every field before the signature, made only
//! from a document that passes the client's own document, quote and log
//! checks, so the values it enrols are ones the TPM signed.

use nsh_attest::consts::ENROLLED_PCRS;
use nsh_crypto::key_digest;

use crate::{boot, encode, Error, Request};

pub fn body(req: &Request<'_>) -> Result<Vec<u8>, Error> {
    let (doc, log) = crate::check::checked(req.doc, Some(req.challenge))?;
    if *doc.ak_public != req.ak {
        return Err(Error::Ak);
    }
    let apps = boot::applications(&log.applications, req.loaders, req.boot_apps)?;
    nsh_wire::names::host(req.host).map_err(|_| Error::Value("host name"))?;
    if req.fleet.iter().all(|b| *b == 0) {
        return Err(Error::Value("fleet id"));
    }
    if req.not_after_ms <= req.now_ms {
        return Err(Error::Value("validity"));
    }
    let mut pcrs = [[0u8; 32]; ENROLLED_PCRS.len()];
    for (slot, i) in pcrs.iter_mut().zip(ENROLLED_PCRS) {
        *slot = *doc.pcr(i).ok_or(Error::Replay(i))?;
    }
    let fields = encode::Fields {
        fleet: req.fleet,
        host: req.host,
        ak: req.ak,
        host_keys: key_digest(&req.host_keys[..32], &req.host_keys[32..]),
        pcrs,
        boot_apps: &apps,
        not_after_ms: req.not_after_ms,
        authority_id: req.authority_id,
    };
    Ok(encode::body(&fields))
}
