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

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let sha = resolve_sha();
    println!("cargo:rustc-env=ABOUT_GIT_SHA={sha}");
    model_weights();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../../LICENSE");
    println!("cargo:rerun-if-env-changed=NONOS_BUILD_SHA");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
}

/*
 * Each pinned Qwen tier and the summed length of its files, for the size on
 * its card, read from the Linux personality's pin tables as setup reads
 * them: every family pinned.rs brings in, each part summed. A pin this
 * cannot read stops the build rather than show a tier with no size.
 */
fn model_weights() {
    const MODELS: &str = "../capsule_linux/src/linux/file/models";
    let read = |file: &str| {
        let path = format!("{MODELS}/{file}");
        println!("cargo:rerun-if-changed={path}");
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
    };
    let field = |block: &str, key: &str, end: char| -> String {
        let rest = block.split_once(key).map(|(_, r)| r);
        let value = rest.and_then(|r| r.split_once(end)).map(|(v, _)| v.trim());
        value.unwrap_or_else(|| panic!("a pin without `{key}`: {block}")).to_string()
    };
    let mut tiers: Vec<(String, u64)> = Vec::new();
    let index = read("pinned.rs");
    for line in index.lines().filter_map(|l| l.trim().strip_prefix("use super::pinned_")) {
        let module = line.split("::").next().unwrap_or_default();
        let text = read(&format!("pinned_{module}.rs"));
        let blocks: Vec<&str> = text.split("Pinned {").skip(1).collect();
        assert!(!blocks.is_empty(), "pinned_{module}.rs: no pins found");
        for block in blocks {
            let tier = field(block, "tier: \"", '"');
            let bytes: u64 = field(block, "bytes: ", ',').replace('_', "").parse().unwrap_or_else(|e| {
                panic!("pinned_{module}.rs: tier {tier}: bytes do not parse: {e}")
            });
            match tiers.iter_mut().find(|(t, _)| *t == tier) {
                Some((_, sum)) => *sum += bytes,
                None => tiers.push((tier, bytes)),
            }
        }
    }
    assert!(!tiers.is_empty(), "pinned.rs: no pinned families found");
    let mut out = String::from("pub const MODEL_WEIGHTS: &[(&str, u64)] = &[\n");
    for (tier, bytes) in &tiers {
        out.push_str(&format!("    (\"{tier}\", {bytes}),\n"));
    }
    out.push_str("];\n");
    let dir = env::var("OUT_DIR").expect("cargo sets OUT_DIR");
    fs::write(Path::new(&dir).join("model_weights.rs"), out).expect("write model_weights.rs");
}

fn resolve_sha() -> String {
    if let Ok(sha) = env::var("NONOS_BUILD_SHA") {
        if !sha.trim().is_empty() {
            return sha.trim().chars().take(12).collect();
        }
    }
    if let Ok(sha) = env::var("GITHUB_SHA") {
        if !sha.trim().is_empty() {
            return sha.trim().chars().take(12).collect();
        }
    }
    if let Some(sha) = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()
        .and_then(|o| if o.status.success() { String::from_utf8(o.stdout).ok() } else { None })
        .map(|s| s.trim().to_string())
    {
        if !sha.is_empty() {
            return sha;
        }
    }
    "unknown".into()
}
