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

/*
 * The Qwen tiers setup offers, read from the pins the Linux personality
 * checks model files against, so the sizes setup shows are the pinned
 * files' own. Every family pinned.rs brings in is read; each tier's parts
 * are summed, and the table is written smallest first. A pin this cannot
 * read stops the build rather than leave a tier out.
 */

use std::{env, fs, path::Path};

const MODELS: &str = "../capsule_linux/src/linux/file/models";

fn main() {
    let index = read("pinned.rs");
    let mut tiers: Vec<(String, u64)> = Vec::new();
    for line in index.lines().filter_map(|l| l.trim().strip_prefix("use super::pinned_")) {
        let module = line.split("::").next().unwrap_or_default();
        let text = read(&format!("pinned_{module}.rs"));
        let blocks: Vec<&str> = text.split("Pinned {").skip(1).collect();
        assert!(!blocks.is_empty(), "pinned_{module}.rs: no pins found");
        for block in blocks {
            let tier = after(block, "tier: \"", '"');
            let bytes: u64 =
                after(block, "bytes: ", ',').replace('_', "").parse().unwrap_or_else(|e| {
                    panic!("pinned_{module}.rs: tier {tier}: bytes do not parse: {e}")
                });
            match tiers.iter_mut().find(|(t, _)| *t == tier) {
                Some((_, sum)) => *sum += bytes,
                None => tiers.push((tier, bytes)),
            }
        }
    }
    assert!(!tiers.is_empty(), "pinned.rs: no pinned families found");
    tiers.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    let mut out = String::from("pub const PINNED: &[(&[u8], u64)] = &[\n");
    for (tier, bytes) in &tiers {
        out.push_str(&format!("    (b\"{tier}\", {bytes}),\n"));
    }
    out.push_str("];\n");
    let dir = env::var("OUT_DIR").expect("cargo sets OUT_DIR");
    fs::write(Path::new(&dir).join("qwen_pins.rs"), out).expect("write qwen_pins.rs");
    println!("cargo:rerun-if-changed=build.rs");
}

fn read(file: &str) -> String {
    let path = format!("{MODELS}/{file}");
    println!("cargo:rerun-if-changed={path}");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/* The text between `key` and the next `end`, or a stopped build. */
fn after(block: &str, key: &str, end: char) -> String {
    let rest = block.split_once(key).map(|(_, r)| r);
    let value = rest.and_then(|r| r.split_once(end)).map(|(v, _)| v.trim());
    value.unwrap_or_else(|| panic!("a pin without `{key}`: {block}")).to_string()
}
