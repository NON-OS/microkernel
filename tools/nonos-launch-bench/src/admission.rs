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

//! Certificate and manifest verification, as `preflight::run` calls them,
//! on every committed capsule. The manifest's payload is the capsule's ELF,
//! which is not committed: the manifest check runs to its payload step and
//! stops there with PayloadHashMismatch, after both publisher signatures
//! have verified. The payload hash is timed in `image`. Each is timed cold
//! (the admission cache emptied before every repeat, as the first spawn of
//! an image after boot) and warm (the same image spawned again).

use crate::inputs::Inputs;
use crate::security::capsule_manifest::{verify_with_publisher, ManifestVerifyError};
use crate::security::nonos_id_cert::{decode as decode_cert, verify as verify_cert, NONOS_PRODUCTION_POLICY};
use crate::security::nonos_trust_anchor::decode as decode_policy;
use crate::security::admission_cache::forget_all;
use crate::stats::{row, time, Spread};

pub fn run(inputs: &Inputs) {
    let policy = match decode_policy(&inputs.policy) {
        Ok(p) => p,
        Err(e) => {
            println!("trust anchor policy does not decode: {e:?}");
            return;
        }
    };
    let mut decode = Vec::new();
    let mut cert = Vec::new();
    let mut manifest = Vec::new();
    let mut refused = Vec::new();
    let mut cert_warm = Vec::new();
    let mut manifest_warm = Vec::new();
    for c in &inputs.capsules {
        decode.push(time(|| {
            let _ = std::hint::black_box(decode_cert(&c.cert));
            let _ = std::hint::black_box(crate::security::capsule_manifest::decode(&c.manifest));
        }));
        let Ok(id) = verify_cert(&c.cert, &policy, &NONOS_PRODUCTION_POLICY, None) else {
            refused.push(format!("{} cert", c.name));
            continue;
        };
        cert.push(time(|| {
            forget_all();
            let _ = std::hint::black_box(verify_cert(&c.cert, &policy, &NONOS_PRODUCTION_POLICY, None));
        }));
        cert_warm.push(time(|| {
            let _ = std::hint::black_box(verify_cert(&c.cert, &policy, &NONOS_PRODUCTION_POLICY, None));
        }));
        let Ok(parsed) = decode_cert(&c.cert) else { continue };
        let check = || {
            verify_with_publisher(
                &c.manifest,
                &c.cert,
                &parsed,
                &id,
                &policy,
                &NONOS_PRODUCTION_POLICY,
                &[],
                "",
                0,
                &[],
            )
        };
        match check() {
            Err(ManifestVerifyError::PayloadHashMismatch) => {}
            other => {
                refused.push(format!("{} manifest {:?}", c.name, other.err()));
                continue;
            }
        }
        manifest.push(time(|| {
            forget_all();
            let _ = std::hint::black_box(check());
        }));
        manifest_warm.push(time(|| {
            let _ = std::hint::black_box(check());
        }));
    }
    summary("cert + manifest decode", &decode);
    summary("cert verify (Ed25519 + ML-DSA-65)", &cert);
    summary("manifest verify to payload (Ed25519 + ML-DSA-65)", &manifest);
    summary("cert verify, acceptance remembered", &cert_warm);
    summary("manifest verify, acceptance remembered", &manifest_warm);
    for r in &refused {
        println!("  not timed: {r}");
    }
}

fn summary(what: &str, per_capsule: &[Spread]) {
    if per_capsule.is_empty() {
        println!("{what:<44} n=0");
        return;
    }
    let mut med: Vec<u128> = per_capsule.iter().map(|s| s.median_ns).collect();
    med.sort_unstable();
    let s = Spread { median_ns: med[med.len() / 2], p90_ns: med[med.len() * 9 / 10] };
    row(what, per_capsule.len(), &s);
}
