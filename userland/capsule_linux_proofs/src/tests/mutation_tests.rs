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

//! Seeded mutation of every reader the Debian and pacman installs added,
//! not coverage-guided (no libFuzzer is vendored). Damaged .debs, indexes,
//! Releases and desc records must return, in a debug build, without a panic.

use super::deb_chain_tests::read;
use crate::install::deb::ar::members;
use crate::install::deb::packages::stanzas;
use crate::install::deb::release::sums;
use crate::install::pacman::desc::records;
use crate::install::tar::walk;
use crate::install::unpacked::unpacked;

use super::mutation::damage;

const ROUNDS: usize = 1500;

#[test]
fn damaged_debs_never_panic() {
    let mut s = 0xDEB5_EEDu64;
    for deb in [
        "pool/main/n/nonos-hello/nonos-hello_1.0_amd64.deb",
        "pool/main/l/libnonos1/libnonos1_1.0_amd64.deb",
        "pool/main/n/nonos-gz/nonos-gz_1.0_amd64.deb",
    ] {
        let clean = read(deb);
        for _ in 0..ROUNDS {
            let mut v = clean.clone();
            damage(&mut s, &mut v);
            for (_, body) in members(&v).unwrap_or_default() {
                let _ = unpacked(body).map(|t| walk(&t));
            }
        }
    }
}

#[test]
fn damaged_indexes_never_panic() {
    let mut s = 0x1DE7_5EEDu64;
    let desc =
        b"%FILENAME%\nx.pkg.tar.zst\n\n%NAME%\nx\n\n%VERSION%\n1-1\n\n%DEPENDS%\na>=1\n".to_vec();
    for clean in [read("dists/nonos/main/binary-amd64/Packages"), read("dists/nonos/Release"), desc]
    {
        for _ in 0..ROUNDS {
            let mut v = clean.clone();
            damage(&mut s, &mut v);
            let text = String::from_utf8_lossy(&v);
            let _ = (stanzas(&text), sums(&text), records(&text));
        }
    }
}

#[test]
fn a_damaged_signal_frame_never_panics_returning() {
    use crate::sigframe::{build, returned};
    let mut s = 0x516E_A100u64;
    let mut base = [0u64; 18];
    base[15] = 0x7fff_ff00_0000;
    let (_, buf, _) = build(&base, 0x4000, 0x4008, 11, 0).expect("frame");
    for _ in 0..ROUNDS {
        let mut v = buf.clone();
        damage(&mut s, &mut v);
        // rt_sigreturn reads the ucontext at the guest's rsp: any bytes there.
        let _ = returned(&v);
        let _ = v.first().map(|_| returned(&v[v.len().min(8)..]));
    }
}
