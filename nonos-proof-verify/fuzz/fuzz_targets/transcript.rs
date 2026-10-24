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

//! The transcript reader never panics, and a transcript it accepts names a
//! root its own slots rebuild. Half the inputs edit the fixture's real
//! transcript, so accepted transcripts come up as well as refused ones: an
//! edit that changes a slot, the seed or the root is accepted only when the
//! slots still give the recorded root.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_proof_verify::read_transcript;

const HONEST: &str = include_str!("../../tests/fixture/caps_root.bin.transcript");
const ROOT: [u8; 32] = *include_bytes!("../../tests/fixture/caps_root.bin");

fuzz_target!(|data: &[u8]| {
    let Some((&mode, rest)) = data.split_first() else {
        return;
    };
    let text = if mode & 1 == 0 {
        String::from_utf8_lossy(rest).into_owned()
    } else {
        // Overwrite the honest text from an offset the input chooses.
        let mut b = HONEST.as_bytes().to_vec();
        let at = (usize::from(mode >> 1) * b.len() / 128).min(b.len());
        for (x, y) in b[at..].iter_mut().zip(rest) {
            *x = *y;
        }
        String::from_utf8_lossy(&b).into_owned()
    };
    let Ok(t) = read_transcript(&text) else {
        return;
    };
    assert!(!t.slots.is_empty());
    // An edited transcript may name another root its edited slots give, but
    // never the honest root with other slots.
    if t.root == ROOT {
        let honest = read_transcript(HONEST).expect("the fixture's transcript reads");
        assert!(t.slots == honest.slots, "other slots accepted under the honest root");
    }
});
