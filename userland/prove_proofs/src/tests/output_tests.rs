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

//! The output file reads back as written, and a reader refuses every cut,
//! extension, word or length that is not the format's. No proof is made here:
//! the proof bytes are opaque to the format.

use nonos_device_attest::Proof;

use crate::assemble::error::Refusal;
use crate::assemble::output::{encode, OUTPUT_HEAD, OUTPUT_MAX};
use crate::assemble::output_read::decode;
use crate::fixture::{device, field_p};

fn with(b: &[u8], at: usize, v: &[u8]) -> Vec<u8> {
    let mut g = b.to_vec();
    g[at..at + v.len()].copy_from_slice(v);
    g
}

fn out(proof_len: usize) -> Result<Vec<u8>, Refusal> {
    let (st, _) = device().assemble().expect("an enrolled device");
    encode(&st, &Proof { bytes: vec![0xA5; proof_len] })
}

#[test]
fn the_output_reads_back_as_written() {
    let (st, _) = device().assemble().expect("an enrolled device");
    let b = out(300).expect("encoded");
    assert_eq!((b.len(), &b[..8]), (OUTPUT_HEAD + 300, &b"NZKDPRF1"[..]));
    let back = decode(&b).expect("decoded");
    assert_eq!((back.statement, back.proof.bytes), (st, vec![0xA5; 300]));
    assert!(decode(&out(OUTPUT_MAX - OUTPUT_HEAD).expect("the largest")).is_some());
    assert_eq!(out(OUTPUT_MAX - OUTPUT_HEAD + 1), Err(Refusal::OutputSize));
}

#[test]
fn every_cut_and_extension_is_refused() {
    let b = out(40).expect("encoded");
    for n in 0..b.len() {
        assert!(decode(&b[..n]).is_none(), "cut at {n}");
    }
    assert!(decode(&[b.clone(), vec![0]].concat()).is_none());
    for i in 0..8 {
        assert!(decode(&with(&b, i, &[b[i] ^ 0x01])).is_none(), "magic byte {i}");
    }
}

#[test]
fn a_word_depth_or_length_that_is_not_the_format_is_refused() {
    let b = out(40).expect("encoded");
    let words = (8..104).step_by(8).chain((108..188).step_by(8));
    for at in words {
        assert!(decode(&with(&b, at, &field_p().to_le_bytes())).is_none(), "word at {at}");
    }
    for depth in [0u32, 21, u32::MAX] {
        assert!(decode(&with(&b, 104, &depth.to_le_bytes())).is_none(), "depth {depth}");
    }
    for len in [0u32, 39, 41, u32::MAX] {
        assert!(decode(&with(&b, 188, &len.to_le_bytes())).is_none(), "length {len}");
    }
}
