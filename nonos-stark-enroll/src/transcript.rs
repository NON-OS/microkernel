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

//! The auditor's transcript. Nothing in a policy tree is secret, so all of it is
//! written down, and the root must be recomputable from this file alone.

use crate::context::{BOOT_EPOCH, DEPTH, POLICY_EPOCH};
use crate::io::{hex, unhex32};
use crate::policy::{enroll, Slot};

const HEADER: &str = "nonos-policy-transcript v2";

pub fn render(slots: &[Slot], pad_seed: &[u8; 32], root: &[u8; 32]) -> String {
    let mut s = format!("{HEADER}\n");
    s += &format!("tool nonos-stark-enroll {}\n", env!("CARGO_PKG_VERSION"));
    s += &format!("source {}\n", option_env!("NONOS_SOURCE_REV").unwrap_or("unrecorded"));
    s += &format!("depth {DEPTH}\npolicy_epoch {POLICY_EPOCH}\nboot_epoch {BOOT_EPOCH}\n");
    s += &format!("pad_seed {}\n", hex(pad_seed));
    for (i, slot) in slots.iter().enumerate() {
        s += &match slot {
            Slot::Kernel(h) => format!("slot {i} kernel {}\n", hex(h)),
            Slot::Bootloader(h) => format!("slot {i} bootloader {}\n", hex(h)),
            Slot::Capsule(h, caps) => format!("slot {i} capsule {} caps {caps:#018x}\n", hex(h)),
        };
    }
    s + &format!("root {}\n", hex(root))
}

/// Rebuild the root from a transcript and check it against the one it records.
pub fn recompute(text: &str) -> Result<[u8; 32], String> {
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("not a policy transcript".into());
    }
    let (mut slots, mut recorded, mut seed) = (Vec::new(), None, None);
    for line in lines {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f.as_slice() {
            ["slot", i, "kernel", h] => {
                expect_index(i, slots.len())?;
                slots.push(Slot::Kernel(unhex32(h)?));
            }
            ["slot", i, "bootloader", h] => {
                expect_index(i, slots.len())?;
                slots.push(Slot::Bootloader(unhex32(h)?));
            }
            ["slot", i, "capsule", h, "caps", c] => {
                expect_index(i, slots.len())?;
                let caps = u64::from_str_radix(c.trim_start_matches("0x"), 16)
                    .map_err(|_| format!("bad caps {c}"))?;
                slots.push(Slot::Capsule(unhex32(h)?, caps));
            }
            ["depth", d] if *d != DEPTH.to_string() => return Err(format!("depth {d}")),
            ["policy_epoch", e] if *e != POLICY_EPOCH.to_string() => {
                return Err(format!("epoch {e}"))
            }
            ["boot_epoch", e] if *e != BOOT_EPOCH.to_string() => return Err(format!("epoch {e}")),
            ["pad_seed", p] => seed = Some(unhex32(p)?),
            ["root", r] => recorded = Some(unhex32(r)?),
            _ => {}
        }
    }
    let root = enroll(&slots, &seed.ok_or("no pad seed recorded")?)?.root;
    match recorded {
        Some(r) if r == root => Ok(root),
        Some(r) => Err(format!("recorded root {} but the slots give {}", hex(&r), hex(&root))),
        None => Err("no root recorded".into()),
    }
}

fn expect_index(i: &str, want: usize) -> Result<(), String> {
    (i.parse::<usize>() == Ok(want)).then_some(()).ok_or(format!("slot {i} out of order"))
}
