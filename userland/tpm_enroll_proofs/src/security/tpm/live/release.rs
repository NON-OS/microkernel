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

//! A release as the release tool approves it: `tools/nonos-policy-approve`'s
//! own functions give PCR 9 and the digest to sign, and the test signs that
//! digest prehashed, as the tool does, with a test P-256 key.

use std::process::Command;

use p256::ecdsa::signature::hazmat::PrehashSigner;
use p256::ecdsa::{Signature, SigningKey};
use sha2::{Digest, Sha256};

use super::tools::hex;
use crate::security::tpm::device_secret::Approval;

const TOOL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tools/nonos-policy-approve");

const BRIDGE: &str = "import importlib.machinery as m, importlib.util as u, sys
l = m.SourceFileLoader('npa', sys.argv[1]); t = u.module_from_spec(u.spec_from_loader('npa', l))
l.exec_module(t); k, r = bytes.fromhex(sys.argv[2]), bytes.fromhex(sys.argv[3])
p = t.kernel_pcr9(k, r); print(p.hex(), t.a_hash(t.approved_policy(p)).hex())";

fn bytes32(h: &str) -> [u8; 32] {
    let v: Vec<u8> = (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i + 2], 16).expect("hex")).collect();
    v.try_into().expect("32 bytes")
}

/// The release of the kernel measured `kernel` under root `root`: the value the
/// loader extends PCR 9 with, PCR 9 after it, and the digest the release signs.
pub struct Release {
    pub extend: [u8; 32],
    pub pcr9: [u8; 32],
    pub a_hash: [u8; 32],
}

pub fn release(kernel: &[u8; 32], root: &[u8; 32]) -> Release {
    let out = Command::new("python3").args(["-c", BRIDGE, TOOL, &hex(kernel), &hex(root)]).output().expect("python3");
    assert!(out.status.success(), "release tool: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8(out.stdout).expect("text");
    let (pcr9, a_hash) = text.trim().split_once(' ').expect("two values");
    let measurement = Sha256::digest([&kernel[..], &root[..]].concat());
    Release { extend: Sha256::digest(measurement).into(), pcr9: bytes32(pcr9), a_hash: bytes32(a_hash) }
}

/// The release key `seed` approving a release.
pub fn approve(seed: u8, r: &Release) -> Approval {
    let key = SigningKey::from_bytes(&[seed; 32].into()).expect("key");
    let sig: Signature = key.sign_prehash(&r.a_hash).expect("signed");
    let point = key.verifying_key().to_encoded_point(false);
    let part = |b: &[u8]| -> [u8; 32] { b.try_into().expect("32 bytes") };
    Approval {
        key_x: part(point.x().expect("x")),
        key_y: part(point.y().expect("y")),
        sig_r: sig.r().to_bytes().into(),
        sig_s: sig.s().to_bytes().into(),
    }
}
