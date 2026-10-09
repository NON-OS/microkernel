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

//! Enrollment tool. Commits capsule and kernel contexts to one policy tree and
//! emits each slot's path trailer, the trailer the attestation gates fold. The
//! tree is padded to the gates' depth with a leaf of the Pad kind, which no gate
//! asks for; every trailer is re-checked with the gates' own check before it is
//! written; and the whole tree is written down as a transcript, from which the
//! root can be rebuilt with nothing else.

mod commands;
mod context;
mod io;
mod measure;
mod policy;
mod selftest;
mod selftest_stark;
mod slot;
mod stark;
mod transcript;
mod variants;
mod verify;

fn usage() -> ! {
    io::die(
        "usage:\n  \
         nonos-stark-enroll selftest\n  \
         nonos-stark-enroll kernel <kernel-image> <root.bin> <trailer.bin>\n  \
         nonos-stark-enroll bootloader <bootloader.efi> <root.bin> <trailer.bin>\n  \
         nonos-stark-enroll capsules <root.bin> <CAPS:image:trailer-out> ...\n  \
         nonos-stark-enroll verify <root.bin> <CAPS:image:trailer> ...\n  \
         nonos-stark-enroll verify-kernel <root.bin> <kernel-image> <trailer>\n  \
         nonos-stark-enroll verify-bootloader <root.bin> <bootloader.efi> <trailer>\n  \
         nonos-stark-enroll recompute <root.bin.transcript>\n  \
         nonos-stark-enroll refusal-variants <root.bin> <CAPS:image:trailer> <outdir>\n  \
         nonos-stark-enroll authenticode <image.efi>",
    )
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match (a.get(1).map(String::as_str), a.len()) {
        (Some("selftest"), 2) => selftest::run(),
        (Some("kernel"), 5) => commands::kernel(&a[2], &a[3], &a[4]),
        (Some("bootloader"), 5) => commands::bootloader(&a[2], &a[3], &a[4]),
        (Some("capsules"), n) if n >= 4 => commands::capsules(&a[2], &a[3..]),
        (Some("verify"), n) if n >= 4 => verify::verify_capsules(&a[2], &a[3..]),
        (Some("verify-kernel"), 5) => verify::verify_kernel(&a[2], &a[3], &a[4]),
        (Some("verify-bootloader"), 5) => verify::verify_bootloader(&a[2], &a[3], &a[4]),
        (Some("recompute"), 3) => commands::recompute_file(&a[2]),
        (Some("refusal-variants"), 5) => variants::run(&a[2], &a[3], &a[4]),
        (Some("authenticode"), 3) => println!("{}", io::hex(&measure::authenticode_of(&a[2]))),
        _ => usage(),
    }
}
