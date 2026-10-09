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

//! The kernel's own decoders run first. A manifest or a certificate the spawn
//! gate would refuse is refused here too, so the signer and the build's self
//! check never approve bytes the booted kernel turns away.

use crate::error::SignError;
use crate::verify::decoded::{DecodedCert, DecodedManifest};

fn refused<E: core::fmt::Debug>(what: &str, e: E) -> SignError {
    SignError::KeyFileShape(format!("the kernel refuses this {what}: {e:?}"))
}

pub fn decode_manifest(bytes: &[u8]) -> Result<DecodedManifest, SignError> {
    admission_proofs::capsule_manifest::decode::decode(bytes)
        .map_err(|e| refused("manifest", e))?;
    super::manifest::decode_manifest(bytes)
}

pub fn decode_cert(bytes: &[u8]) -> Result<DecodedCert, SignError> {
    admission_proofs::nonos_id_cert::decode::decode(bytes)
        .map_err(|e| refused("certificate", e))?;
    super::cert::decode_cert(bytes)
}
