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

//! `release.kernel-signature`: the kernel carries the release signature the
//! loader checks before it runs it, and both halves verify, under the public
//! keys the loader is built with (tools/nix/image.nix). The signed message is
//! BLAKE3 of the kernel's bytes and the rollback index (sign-kernel), and the
//! rollback index the image carries must be the one the record names.

use super::kernel::parse;
use super::kernel_keys::KernelKeys;
use super::read::read_regular;
use super::record::Item;
use super::resolve::resolve;
use crate::check::Check;
use crate::ctx::Ctx;
use std::path::Path;

pub const ID: &str = "release.kernel-signature";
const MAX_KERNEL: u64 = 1 << 30;

pub fn check(ctx: &mut Ctx) -> Check {
    let (Some(rec), Some(tree)) = (&ctx.record, &ctx.tree) else {
        return Check::not_run(
            ID,
            "needs the record and the tree that holds the kernel's public keys",
        );
    };
    let kernel = rec.sealed.iter().find(|s| s.name == "kernel" && matches!(s.item, Item::File(_)));
    let Some(k) = kernel else {
        return Check::fail(ID, "the record seals no kernel file");
    };
    let bytes = resolve(&ctx.release, &k.rel, false).and_then(|at| read_regular(&at, MAX_KERNEL));
    match bytes.and_then(|b| verify(&b, tree, rec.rollback_index)) {
        Ok(why) => Check::pass(ID, why),
        Err(e) => Check::fail(ID, format!("{}: {e}", k.rel)),
    }
}

pub(crate) fn verify(bytes: &[u8], tree: &Path, rollback: Option<u32>) -> Result<String, String> {
    let a = parse(bytes)?;
    if rollback.is_some_and(|r| r != a.rollback) {
        return Err(format!("it carries rollback index {}, the record {rollback:?}", a.rollback));
    }
    KernelKeys::load(tree)?.verify(&a)?;
    Ok(format!(
        "Ed25519 and ML-DSA-65 verify under the tree's kernel keys, rollback {}",
        a.rollback
    ))
}
