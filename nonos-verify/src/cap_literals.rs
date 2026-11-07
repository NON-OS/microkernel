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

// A process's capability bits are `Capability::X.bit()`
// (src/capabilities/types/defs.rs) and nothing else. A mask written by hand
// as `1 << n` names whatever bit n is today: src/process/core/isolation.rs
// called 1 << 10 Network and 1 << 13 Device while 10 was RegisterService and
// 13 GraphicsSurfaceMap, so isolating a process would have stripped its
// graphics and left its network. This fails on a new one. Where capability
// tokens are handled, a constant or variable named for a capability may not
// be a shift literal, and the process-capability calls may not take a
// literal mask. a work area.

use std::fs;
use std::path::{Path, PathBuf};

/// The parts of the kernel that hold or test process capability bits.
const AREAS: &[&str] = &[
    "src/syscall/",
    "src/process/",
    "src/kernel_core/",
    "src/services/",
    "src/userspace/",
    "src/ipc/",
    "src/fs/",
    "src/hardware/",
    "src/security/",
];


/// Register bits that happen to be called CAP: hardware's, not a token's.
const HARDWARE: &[&str] = &[
    // PCIe Device Capabilities: Function Level Reset supported (bit 28).
    "FLR_CAP",
];

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

fn shift_literal(code: &str) -> bool {
    ["1 << ", "1u64 << ", "1_u64 << ", "(1 << ", "1<<"].iter().any(|p| code.contains(p))
}

fn names_a_capability(code: &str) -> bool {
    let upper = code.to_ascii_uppercase();
    // A screaming-case name ending or starting in CAP(S), or a caps mask.
    ["_CAP:", "_CAPS:", "_CAP =", "_CAPS =", "CAPS_BITS", "CAP_MASK", "CAPS_MASK"].iter().any(|n| upper.contains(n))
        || code.contains("caps_bits")
}

fn literal_mask_call(code: &str) -> bool {
    for call in ["caps::revoke(", "caps::grant(", "caps::has(", "proc_caps::revoke(", "proc_caps::grant("] {
        if let Some(at) = code.find(call) {
            let args = &code[at + call.len()..];
            let second = args.split(',').nth(1).unwrap_or("").trim_start();
            if second.starts_with("0x") || second.starts_with("1 <<") || second.starts_with("(1 <<") {
                return true;
            }
        }
    }
    false
}

fn findings(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    let mut out = Vec::new();
    for f in &files {
        let rel = f.strip_prefix(root).unwrap_or(f).to_string_lossy().replace('\\', "/");
        if !AREAS.iter().any(|a| rel.starts_with(a)) {
            continue;
        }
        let text = fs::read_to_string(f).unwrap_or_default();
        for (n, line) in text.lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") || code.starts_with('*') || code.starts_with("/*") {
                continue;
            }
            let hardware = HARDWARE.iter().any(|h| code.contains(h));
            if (!hardware && shift_literal(code) && names_a_capability(code)) || literal_mask_call(code) {
                out.push(format!("{rel}:{}: {code}", n + 1));
            }
        }
    }
    out
}

#[test]
fn no_capability_bit_is_written_by_hand() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let found = findings(&root);
    assert!(found.is_empty(), "capability bits written by hand:\n{}", found.join("\n"));
}

#[test]
fn the_isolation_constants_would_have_been_caught() {
    let lines = [
        "    const NETWORK_CAP: u64 = 1 << 10;",
        "    proc_caps::revoke(pid, 0x3c00).ok_or(\"Process not found\")?;",
    ];
    assert!(shift_literal(lines[0].trim_start()) && names_a_capability(lines[0].trim_start()));
    assert!(literal_mask_call(lines[1].trim_start()));
    assert!(!names_a_capability("    pub const COUNT_SIZE_CAP_BIT: u32 = 13;"));
}

/// The variant names of the one capability enum.
fn real_names(root: &Path) -> Vec<String> {
    let defs = fs::read_to_string(root.join("src/capabilities/types/defs.rs")).unwrap_or_default();
    defs.lines()
        .filter_map(|l| {
            let l = l.trim_start();
            let l = l.rsplit("] ").next().unwrap_or(l);
            let (name, rest) = l.split_once(" = ")?;
            (rest.trim_start().starts_with("1 <<") && name.chars().all(|c| c.is_ascii_alphanumeric()))
                .then(|| name.to_string())
        })
        .collect()
}

#[test]
fn there_is_one_capability_enum() {
    // src/security/policy/capability had a second `Capability` with its own
    // bit numbers, glob-re-exported as `crate::security::Capability`. No type
    // of that name, and no bit enum that reuses two or more of the real
    // variant names, may stand outside src/capabilities.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let real = real_names(&root);
    assert!(real.len() > 30, "the capability enum was not read: {real:?}");
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    let mut found = Vec::new();
    for f in &files {
        let rel = f.strip_prefix(&root).unwrap_or(f).to_string_lossy().replace('\\', "/");
        if rel.starts_with("src/capabilities/") {
            continue;
        }
        let text = fs::read_to_string(f).unwrap_or_default();
        for (n, line) in text.lines().enumerate() {
            let code = line.trim_start();
            // A struct of that name may stand elsewhere (the contract's
            // `Capability` is a witness that the check ran, holding a token,
            // not bits); an enum or alias of that name may not.
            for kind in ["enum ", "type "] {
                if let Some(rest) = code.strip_prefix("pub ").unwrap_or(code).strip_prefix(kind) {
                    let name: String = rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
                    if name == "Capability" || name == "CapabilityType" {
                        found.push(format!("{rel}:{}: {code}", n + 1));
                    }
                }
            }
        }
        let reused = text
            .lines()
            .filter_map(|l| l.trim_start().split_once(" = "))
            .filter(|(name, rest)| rest.trim_start().starts_with("1 <<") && real.iter().any(|r| r == name))
            .count();
        if reused >= 2 {
            found.push(format!("{rel}: {reused} bit variants named as capabilities"));
        }
    }
    assert!(found.is_empty(), "a second capability enum:\n{}", found.join("\n"));
}
