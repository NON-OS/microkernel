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

//! The build receipt the seal writes beside the release (schema
//! `nonos.build.receipt.v1`, scripts/build_receipt.py): the commit it was
//! built from, whether the roots it measured are embedded, and the digest of
//! every artifact it measured. Read as untrusted, like the record.

use super::receipt_parts::{artifact, is_commit};
use serde_json::Value;

pub const FILE: &str = "build-receipt.json";
pub const SCHEMA: &str = "nonos.build.receipt.v1";
pub const MAX_BYTES: u64 = 1 << 20;
const MAX_ARTIFACTS: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub commit: String,
    pub dirty: bool,
    pub embedded: bool,
    pub policy_in_kernel: bool,
    pub attest_root: Option<bool>,
    /// Each artifact by file name, and its SHA-256 (none: it was missing).
    pub artifacts: Vec<(String, Option<String>)>,
}

pub fn parse(bytes: &[u8]) -> Result<Receipt, String> {
    let v: Value = serde_json::from_slice(bytes).map_err(|e| format!("not JSON ({e})"))?;
    let at = |path: &[&str]| path.iter().try_fold(&v, |v, k| v.get(k));
    if at(&["schema"]).and_then(Value::as_str) != Some(SCHEMA) {
        return Err(format!("its schema is not `{SCHEMA}`"));
    }
    let commit = at(&["source", "commit"]).and_then(Value::as_str).filter(|c| is_commit(c));
    let commit = commit.ok_or("`source.commit` is not a commit id")?.to_string();
    let dirty = at(&["source", "dirty"])
        .and_then(Value::as_bool)
        .ok_or("`source.dirty` is not true or false")?;
    let policy = at(&["roots", "capsule_policy", "embedded_in_kernel"]).and_then(Value::as_bool);
    let policy_in_kernel = policy.ok_or("no `roots.capsule_policy.embedded_in_kernel`")?;
    let attest_root = match at(&["roots", "kernel_attest"]) {
        None | Some(Value::Null) => None,
        Some(a) => {
            Some(a.get("ok").and_then(Value::as_bool).ok_or("bad `roots.kernel_attest.ok`")?)
        }
    };
    let embedding = at(&["verification", "root_embedding"]).and_then(Value::as_str);
    let embedded = embedding.ok_or("no `verification.root_embedding`")? == "pass";
    let arts = at(&["artifacts"]).and_then(Value::as_object).ok_or("no `artifacts` object")?;
    if arts.is_empty() || arts.len() > MAX_ARTIFACTS {
        return Err("`artifacts` is empty, or far longer than a seal writes".into());
    }
    let artifacts = arts.iter().map(|(path, a)| artifact(path, a)).collect::<Result<_, _>>()?;
    Ok(Receipt { commit, dirty, embedded, policy_in_kernel, attest_root, artifacts })
}
