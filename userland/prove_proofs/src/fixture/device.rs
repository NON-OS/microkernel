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

//! Everything the capsule reads on a machine that should prove: the request,
//! the transcript, the slots record, the EK answers and the secret. A test
//! changes one of them and runs the pure half in the capsule's own order.

use nonos_device_attest::{Registry, Statement, Witness};

use super::{answer, registry, release, request, root_bytes, secret_of, Release};
use super::{NONCE, SECRET, VERIFIER, WINDOW};
use crate::assemble::enrolled::enrolled;
use crate::assemble::error::Refusal;
use crate::assemble::request::parse_request;
use crate::assemble::slots::slots;
use crate::assemble::statement::assemble;

pub struct Device {
    pub request: Vec<u8>,
    pub transcript: Vec<u8>,
    pub record: Vec<u8>,
    pub eks: Vec<Vec<u8>>,
    pub secret: [u8; 32],
    pub registry: Registry,
    pub release: Release,
}

/// The P-256 EK is not enrolled; the RSA one is, so the second is found.
pub fn device() -> Device {
    let (ecc, rsa) = (answer(0xEC), answer(0x5A));
    let reg = registry(&rsa, SECRET);
    let release = release();
    Device {
        request: request(VERIFIER, WINDOW, &NONCE, &root_bytes(&reg)),
        transcript: reg.transcript().into_bytes(),
        record: release.record.to_vec(),
        eks: vec![ecc, rsa],
        secret: secret_of(SECRET),
        registry: reg,
        release,
    }
}

impl Device {
    pub fn assemble(&self) -> Result<(Statement, Witness), Refusal> {
        let req = parse_request(&self.request)?;
        let eks: Vec<&[u8]> = self.eks.iter().map(Vec::as_slice).collect();
        let device = enrolled(&self.transcript, &req, &eks)?;
        let s = slots(&self.record)?;
        assemble(&req, device, s, &self.secret)
    }
}
