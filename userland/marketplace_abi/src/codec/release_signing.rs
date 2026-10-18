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

//! Canonical bytes a publisher signs for one release.

extern crate alloc;

use alloc::vec::Vec;

use super::write_release_ext;
use super::writer::Writer;
use crate::types::CapsuleRelease;

// v2 because the signed bytes gained the trailer hash below.
const RELEASE_SIGNING_DOMAIN_V2: &[u8] = b"NONOS.marketplace.release.v2";
// v3 because they gained the commerce hash, guest policy, chunk tree and
// proof anchors. A v2 signature can never verify as v3, or the reverse.
const RELEASE_SIGNING_DOMAIN_V3: &[u8] = b"NONOS.marketplace.release.v3";

/// The release's own fields pick the domain: a schema 3 release carries
/// `ext` and signs under v3, a schema 2 one does not and signs under v2.
pub fn release_signing_bytes(release: &CapsuleRelease) -> Vec<u8> {
    let mut out = Vec::new();
    let mut w = Writer::new(&mut out);
    let domain = match release.ext {
        Some(_) => RELEASE_SIGNING_DOMAIN_V3,
        None => RELEASE_SIGNING_DOMAIN_V2,
    };
    w.lp_bytes(domain);
    w.lp_string(&release.release_id);
    w.fixed(&release.manifest_hash);
    w.fixed(&release.package_hash);
    w.lp_string(&release.package_url);

    w.u32(release.supported_arches.len() as u32);
    for arch in &release.supported_arches {
        w.lp_string(arch);
    }

    w.u32(release.kernel_abi_min);
    w.u32(release.required_capabilities.len() as u32);
    for cap in &release.required_capabilities {
        w.lp_string(cap);
    }
    w.fixed(&release.zk_trailer_hash);
    if let Some(ext) = &release.ext {
        write_release_ext::write(&mut w, ext);
    }
    out
}
