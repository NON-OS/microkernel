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

use std::path::Path;

use nonos_capsule_sign::algs::ed25519::Ed25519;
use nonos_capsule_sign::algs::mldsa65::MlDsa65;
use nonos_capsule_sign::algs::traits::Verifier;
use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::error::SignError;
use nonos_capsule_sign::keys::read_pub;
use nonos_capsule_sign::release::{canonical_manifest, find_entry, merkle_root};

use super::parse::{decode_hex, parse_hex32, parse_u64};

pub fn run(args: &[String]) -> Result<(), SignError> {
    if args.len() != 9 {
        return Err(SignError::Usage(
            "verify-release <ed25519.pub> <mldsa65.pub> <log> <version> <commit40> <bl_hash> <kernel_hash> <zk_root> <epoch>".into(),
        ));
    }
    let ed = read_pub(Path::new(&args[0]))?;
    let pq = read_pub(Path::new(&args[1]))?;
    if ed.alg != AlgId::Ed25519 || pq.alg != AlgId::MlDsa65 {
        return Err(SignError::Usage("expected an ed25519 pubkey and an ml-dsa-65 pubkey".into()));
    }
    let commit = decode_hex(&args[4])?;
    if commit.len() != 20 {
        return Err(SignError::Usage("commit must be 40 hex chars".into()));
    }
    let mut commit20 = [0u8; 20];
    commit20.copy_from_slice(&commit);
    let manifest = canonical_manifest(
        parse_u64(&args[3])?,
        &commit20,
        &parse_hex32(&args[5])?,
        &parse_hex32(&args[6])?,
        &parse_hex32(&args[7])?,
        parse_u64(&args[8])?,
    );
    let (ed_sig, pq_sig) = find_entry(&args[2], &hex::encode(&manifest))
        .ok_or_else(|| SignError::Usage("release not present in transparency log".into()))?;
    let ed_ok = Ed25519::verify(&ed.bytes, &manifest, &ed_sig)?;
    let pq_ok = MlDsa65::verify(&pq.bytes, &manifest, &pq_sig)?;
    if !ed_ok || !pq_ok {
        return Err(SignError::Usage("signature INVALID: hybrid verify failed".into()));
    }
    println!(
        "VERIFIED release v{}: ed25519 + ml-dsa-65 both valid, present in transparency log",
        &args[3]
    );
    let root = merkle_root(&args[2]).map_err(SignError::Io)?;
    println!("  transparency log root {}", hex::encode(root));
    Ok(())
}
