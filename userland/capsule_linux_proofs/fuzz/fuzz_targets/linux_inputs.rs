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

//! What a Linux guest or a package server hands the personality: Wayland wire
//! messages, a guest ELF, tar and deb archives, base64 and HTTP replies. No
//! reader panics, and what each hands on lies inside what it was given.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_capsule_linux_proofs::image::elf::Elf;
use nonos_capsule_linux_proofs::install::{auth::base64, deb::ar, http_reply, tar};
use nonos_capsule_linux_proofs::wire;

fn inside(outer: &[u8], inner: &[u8]) -> bool {
    let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
    inner.is_empty() || (i >= o && i + inner.len() <= o + outer.len())
}

fuzz_target!(|data: &[u8]| {
    let Some((&which, b)) = data.split_first() else {
        return;
    };
    match which % 6 {
        0 => {
            if let Some((m, used)) = wire::next(b) {
                assert!((8..=b.len()).contains(&used) && inside(&b[..used], m.args));
            }
        }
        1 => {
            let _ = Elf::parse(b);
        }
        2 => {
            let _ = tar::walk(b);
        }
        3 => {
            for (name, body) in ar::members(b).unwrap_or_default() {
                assert!(inside(b, name) && inside(b, body), "an ar member outside the archive");
            }
        }
        4 => {
            if let Some(out) = base64::decode(b) {
                assert!(out.len() <= b.len() / 4 * 3 + 3, "base64 grew");
            }
        }
        _ => {
            let _ = http_reply::framing(b);
            let _ = http_reply::complete(b);
            let _ = http_reply::body(b.to_vec());
        }
    }
});
