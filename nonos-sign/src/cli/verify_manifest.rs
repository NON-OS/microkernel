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

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::error::SignError;
use nonos_capsule_sign::verify::decode::{
    decode_cert, decode_manifest, decode_trust_anchor_policy,
};
use nonos_capsule_sign::verify::{verify_cert, verify_manifest};

const REQUIRED: &[AlgId] = &[AlgId::Ed25519, AlgId::MlDsa65];

pub fn run(av: &[String]) -> Result<(), SignError> {
    let (mp, cp, pp, elf, now_ms) = parse(av)?;
    let mb = fs::read(&mp)?;
    let cb = fs::read(&cp)?;
    let pb = fs::read(&pp)?;
    let dpol = decode_trust_anchor_policy(&pb)?;
    let dcert = decode_cert(&cb)?;
    verify_cert(&dcert, &cb, &dpol, REQUIRED, Some(now_ms))?;
    let dmf = decode_manifest(&mb)?;
    verify_manifest(&dmf, &mb, &dcert, &cb, &dpol, REQUIRED)?;
    // With --elf, the manifest must bind these exact bytes. This is the
    // reuse gate for pipelines that hold no signing seeds: a rebuilt
    // capsule either measures to what the owner enrolled or the build
    // is not the enrolled build and must fail here, by name.
    if let Some(ep) = &elf {
        let eb = fs::read(ep)?;
        let got = *blake3::hash(&eb).as_bytes();
        if got != dmf.payload_hash {
            return Err(SignError::Usage(format!(
                "verify-manifest: {} does not measure to the enrolled payload\n  enrolled {}\n  rebuilt  {}",
                ep.display(),
                hex32(&dmf.payload_hash),
                hex32(&got)
            )));
        }
    }
    println!(
        "manifest {} verifies under cert {} + policy {}",
        mp.display(),
        cp.display(),
        pp.display()
    );
    println!("  namespace              {}", dmf.namespace);
    println!("  version                {}.x.x", dmf.version_major);
    println!("  target_triple          {}", dmf.target_triple);
    println!("  required_caps          0x{:016x}", dmf.required_caps);
    println!("  optional_caps          0x{:016x}", dmf.optional_caps);
    println!("  endpoints              {}", dmf.endpoints.len());
    println!("  publisher signatures   {}", dmf.publisher_signatures.len());
    if elf.is_some() {
        println!("  payload binding        rebuilt ELF measures to the enrolled hash");
    }
    Ok(())
}

fn hex32(b: &[u8; 32]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn parse(av: &[String]) -> Result<(PathBuf, PathBuf, PathBuf, Option<PathBuf>, u64), SignError> {
    let (mut mf, mut ce, mut po, mut el, mut now) = (None, None, None, None, None);
    let mut i = 0;
    while i < av.len() {
        let n = |k: &str| SignError::Usage(format!("verify-manifest: {}", k));
        let v = |k: &str| av.get(i + 1).ok_or_else(|| n(k));
        match av[i].as_str() {
            "--manifest" => {
                mf = Some(PathBuf::from(v("--manifest <path>")?));
                i += 2;
            }
            "--cert" => {
                ce = Some(PathBuf::from(v("--cert <path>")?));
                i += 2;
            }
            "--policy" => {
                po = Some(PathBuf::from(v("--policy <path>")?));
                i += 2;
            }
            "--elf" => {
                el = Some(PathBuf::from(v("--elf <path>")?));
                i += 2;
            }
            "--now-ms" => {
                now = Some(v("--now-ms <n>")?.parse().map_err(|_| n("bad --now-ms"))?);
                i += 2;
            }
            other => return Err(n(&format!("unknown `{}`", other))),
        }
    }
    let now_v = now.unwrap_or_else(|| {
        SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
    });
    Ok((
        mf.ok_or_else(|| SignError::Usage("verify-manifest: missing --manifest".into()))?,
        ce.ok_or_else(|| SignError::Usage("verify-manifest: missing --cert".into()))?,
        po.ok_or_else(|| SignError::Usage("verify-manifest: missing --policy".into()))?,
        el,
        now_v,
    ))
}
