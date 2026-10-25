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

//! The kernel's own PCR 14 events (`src/security/capsule_measure`), in the
//! two-bank log a PC firmware writes: each bank's digest is that bank's hash
//! of the same 65-byte body, and the events replay through the client's
//! whole check.

use super::super::passes;
use crate::build::tcg_event::extend;
use crate::build::Spec;
use crate::kernel_measure::event::EV_NONOS_CAPSULE;
use crate::kernel_measure::event::{body, digest, event, ALG_SHA1, ALG_SHA256, ALG_SHA384};
use crate::kernel_measure::kind::VENDOR;

mod banks;
mod kinds;

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex")).collect()
}

#[test]
fn every_bank_s_digest_is_its_hash_of_the_one_body() {
    let b = body(&[0x11; 32], &[0x22; 32], VENDOR);
    // Computed apart from this code, by another implementation of each hash.
    assert_eq!(digest(ALG_SHA1, &b), Some(hex("b65e18744cc97e09e220c228cad694ede0785ed2")));
    let sha256 = "4a9fde315fe45879b6d5e2ab6ae6582c9909b96c1197602efc6679eb17157d99";
    assert_eq!(digest(ALG_SHA256, &b), Some(hex(sha256)));
    let sha384 = "45599ac8ab4d3aa2b309cdca6b8452f8489913025e7e61e4842ab69034c5f466\
                  80b440e52bd7d30b222a6a2715203ee6";
    assert_eq!(digest(ALG_SHA384, &b), Some(hex(sha384)));
    let (bytes, digests) = event(&[ALG_SHA1, ALG_SHA256], &b).expect("two banks");
    assert_eq!(
        &bytes[..12],
        &[&14u32.to_le_bytes()[..], &EV_NONOS_CAPSULE.to_le_bytes(), &2u32.to_le_bytes()].concat()
            [..]
    );
    assert_eq!(
        digests,
        [(ALG_SHA1, hex("b65e18744cc97e09e220c228cad694ede0785ed2")), (ALG_SHA256, hex(sha256))]
    );
    assert_eq!(&bytes[bytes.len() - 65..], &b[..]);
    assert_eq!(event(&[ALG_SHA256, 0x0012], &b), None, "a bank the kernel cannot hash");
}

#[test]
fn the_kernel_s_events_replay_through_the_whole_check() {
    let mut s = Spec::default();
    for i in 1..=3u8 {
        let b = body(&[i; 32], &[0xC0 + i; 32], VENDOR);
        let (bytes, _) = event(&[ALG_SHA1, ALG_SHA256], &b).expect("two banks");
        s.log.bytes.extend(bytes);
        s.log.pcrs[14] = extend(
            &s.log.pcrs[14],
            &digest(ALG_SHA256, &b).expect("sha256").try_into().expect("32"),
        );
    }
    passes(s);
}
