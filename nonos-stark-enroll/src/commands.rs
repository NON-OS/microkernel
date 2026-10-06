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

//! The enrollment verbs the build drives, one root and trailers per call.

use crate::io::{die, hash_of, hex, os_random, read, write};
use crate::measure::authenticode_of;
use crate::policy::{enroll, Slot};
use crate::transcript::{recompute, render};

/// Enroll slots, write the root, each trailer and the transcript beside the root.
fn emit(slots: &[Slot], root_out: &str, outs: &[&str]) {
    let pad_seed = os_random();
    let policy = enroll(slots, &pad_seed).unwrap_or_else(|e| die(&format!("enroll: {e}")));
    write(root_out, &policy.root);
    write(&format!("{root_out}.transcript"), render(slots, &pad_seed, &policy.root).as_bytes());
    for (out, trailer) in outs.iter().zip(&policy.trailers) {
        write(out, trailer);
    }
    println!("enrolled {} slots under root {}", slots.len(), hex(&policy.root));
}

pub fn kernel(image: &str, root_out: &str, trailer_out: &str) {
    emit(&[Slot::Kernel(hash_of(image))], root_out, &[trailer_out]);
}

/*
 * The bootloader gets a tree of its own, under its Authenticode digest. It cannot
 * share the kernel's: the bootloader embeds the kernel root, so its measurement
 * would depend on a root that depends on it. The release signs this root in
 * boot_root.approval, and the anonymous proof opens it beside the kernel's.
 */
pub fn bootloader(image: &str, root_out: &str, trailer_out: &str) {
    emit(&[Slot::Bootloader(authenticode_of(image))], root_out, &[trailer_out]);
}

/// Each spec is `CAPS:image:path`, CAPS the granted capability word in hex.
pub(crate) fn spec(s: &str) -> (u64, &str, &str) {
    let p: Vec<&str> = s.splitn(3, ':').collect();
    let [caps, image, path] = p.as_slice() else {
        die(&format!("bad spec {s}, want CAPS:image:path"))
    };
    let caps = u64::from_str_radix(caps.trim_start_matches("0x"), 16)
        .unwrap_or_else(|_| die(&format!("bad capability word {caps}")));
    (caps, image, path)
}

pub fn capsules(root_out: &str, specs: &[String]) {
    let parsed: Vec<_> = specs.iter().map(|s| spec(s)).collect();
    let slots: Vec<Slot> =
        parsed.iter().map(|(c, img, _)| Slot::Capsule(hash_of(img), *c)).collect();
    let outs: Vec<&str> = parsed.iter().map(|(_, _, o)| *o).collect();
    emit(&slots, root_out, &outs);
}

pub fn recompute_file(path: &str) {
    let text = String::from_utf8(read(path)).unwrap_or_else(|_| die("transcript is not text"));
    match recompute(&text) {
        Ok(root) => println!("root {} reproduced from {path}", hex(&root)),
        Err(e) => die(&format!("transcript {path}: {e}")),
    }
}
