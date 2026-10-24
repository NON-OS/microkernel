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

//! Which branch protection the kernel crate is compiled with, declared to it
//! as cfgs, so the kernel turns on only what its own code carries: EnIA set
//! over code that does not sign leaves nothing to authenticate, and BTI over
//! code without landing pads faults on the first indirect call.
//!
//! The answer is in the flags cargo hands rustc for this crate, which for a
//! target build are that target's rustflags. A function's first instruction
//! cannot answer it: with frame pointers always kept, every function saves
//! its link register, so under pac-ret every one starts with PACIASP, which
//! is also a landing pad, with or without bti.

pub(crate) fn declare_branch_protection() {
    println!("cargo::rustc-check-cfg=cfg(nonos_bti)");
    println!("cargo::rustc-check-cfg=cfg(nonos_pac_ret)");
    let flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let mut words = flags.split('\x1f');
    let mut value = None;
    while let Some(word) = words.next() {
        let spec = match word {
            "-Z" => words.next().unwrap_or(""),
            w => w.strip_prefix("-Z").unwrap_or(""),
        };
        if let Some(v) = spec.strip_prefix("branch-protection=") {
            value = Some(v.to_string());
        }
    }
    let Some(value) = value else { return };
    for part in value.split(',') {
        if part == "bti" {
            println!("cargo:rustc-cfg=nonos_bti");
        }
        // The kernel installs the A key. pac-ret+b-key signs with IB, which
        // nothing enables, so it is not declared.
        let mut modifiers = part.split('+');
        if modifiers.next() == Some("pac-ret") && modifiers.all(|m| m == "leaf") {
            println!("cargo:rustc-cfg=nonos_pac_ret");
        }
    }
}
