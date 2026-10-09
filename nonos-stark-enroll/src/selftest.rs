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

//! The enroll, trailer and gate loop end to end, including the capability gap
//! and the transcript an auditor rebuilds the root from.

use nonos_attest_path::Kind;

use crate::context::{capsule_context, kernel_context};
use crate::io::{die, hex, os_random};
use crate::policy::{enroll, Slot};
use crate::stark::check_v4;
use crate::transcript::{recompute, render};

pub(crate) fn check(ok: bool, what: &str) {
    if !ok {
        die(&format!("selftest FAILED: {what}"));
    }
}

pub fn run() {
    let h = |b: &[u8]| *blake3::hash(b).as_bytes();
    let (a, b, k) = (h(b"capsule:terminal"), h(b"capsule:net_core"), h(b"kernel image"));
    let bl = h(b"bootloader image");
    let slots =
        [Slot::Capsule(a, 0x7), Slot::Capsule(b, 0x11), Slot::Kernel(k), Slot::Bootloader(bl)];
    let seed = os_random();
    let p = enroll(&slots, &seed).unwrap_or_else(|e| die(&e));
    let cap = |img, caps, t: &[u8]| check_v4(&p.root, Kind::Capsule, &capsule_context(img, caps), t).is_ok();
    let boot = |kind, img, t: &[u8]| check_v4(&p.root, kind, &kernel_context(img), t).is_ok();

    check(cap(&a, 0x7, &p.trailers[0]) && cap(&b, 0x11, &p.trailers[1]), "enrolled capsules");
    check(boot(Kind::Kernel, &k, &p.trailers[2]), "kernel");
    check(boot(Kind::Bootloader, &bl, &p.trailers[3]), "bootloader");
    check(!boot(Kind::Kernel, &bl, &p.trailers[3]), "the bootloader's slot admitted as a kernel");
    crate::selftest_stark::run(&p.root, &a, &b, &p.trailers);

    /*
     * The gap this tree closes. The old leaf was the image alone and a trailer
     * could be built by anyone, so an enrolled capsule was admitted under any
     * capability word. Every path in the tree, presented for capsule A at 0xFF,
     * must now be refused.
     */
    for (i, t) in p.trailers.iter().enumerate() {
        check(!cap(&a, 0xFF, t), &format!("capsule A at 0xFF admitted on slot {i}'s path"));
    }
    check(!cap(&h(b"capsule:rogue"), 0x7, &p.trailers[0]), "a rogue image admitted");

    /*
     * The pad seed is part of the root: the same slots under another seed give
     * another root, and a transcript without its seed reproduces nothing.
     */
    let other = enroll(&slots, &os_random()).unwrap_or_else(|e| die(&e));
    check(other.root != p.root, "the pad seed left the root unchanged");

    let text = render(&slots, &seed, &p.root);
    check(recompute(&text) == Ok(p.root), "the root from its own transcript");
    let forged = text.replace("caps 0x0000000000000007", "caps 0x00000000000000ff");
    check(recompute(&forged).is_err(), "a transcript with a changed capability word");
    let unseeded: String =
        text.lines().filter(|l| !l.starts_with("pad_seed")).map(|l| l.to_owned() + "\n").collect();
    check(recompute(&unseeded).is_err(), "a transcript without its pad seed");

    println!("selftest OK: {} slots under root {}", slots.len(), hex(&p.root));
}
