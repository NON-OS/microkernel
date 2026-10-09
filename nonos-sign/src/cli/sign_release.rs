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
use nonos_capsule_sign::algs::traits::Signer;
use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::error::SignError;
use nonos_capsule_sign::keys::read_seed;
use nonos_capsule_sign::release::{append_entry, canonical_manifest, merkle_root};

use super::parse::{decode_hex, parse_hex32, parse_u64};

pub fn run(args: &[String]) -> Result<(), SignError> {
    if args.len() != 9 {
        return Err(SignError::Usage(
            "sign-release <ed25519.seed> <mldsa65.seed> <log> <version> <commit40> <bl_hash> <kernel_hash> <zk_root> <epoch>".into(),
        ));
    }
    let ed = read_seed(Path::new(&args[0]))?;
    let pq = read_seed(Path::new(&args[1]))?;
    if ed.alg != AlgId::Ed25519 || pq.alg != AlgId::MlDsa65 {
        return Err(SignError::Usage("expected an ed25519 seed and an ml-dsa-65 seed".into()));
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
    let ed_sig = Ed25519::sign(&ed.bytes, &manifest)?;
    let pq_sig = MlDsa65::sign_reproducible(&pq.bytes, &manifest)?;
    append_entry(&args[2], &manifest, &ed_sig, &pq_sig).map_err(SignError::Io)?;
    println!("signed release v{} commit {}", &args[3], &args[4]);
    println!("  ed25519  sig {} bytes", ed_sig.len());
    println!("  ml-dsa65 sig {} bytes", pq_sig.len());
    let root = merkle_root(&args[2]).map_err(SignError::Io)?;
    println!("  transparency log root {}", hex::encode(root));
    Ok(())
}
