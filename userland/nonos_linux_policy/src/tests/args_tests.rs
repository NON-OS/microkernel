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

//! A listed-argument block is read in one spelling, and a person's own word
//! names a listed option only by its whole name.

use crate::args::{check, name, names, words, ArgsError as E};

/// The catalogue's listed_args.py writes these same bytes for these words.
const PINNED: &str = "4e4c58415247303102232d2d6461656d6f6e2d616464726573733d6e6f64652e6578616d706c653a3138303831122d2d756e747275737465642d6461656d6f6e";

pub(super) fn block(words: &[&[u8]]) -> Vec<u8> {
    let mut b = b"NLXARG01".to_vec();
    b.push(words.len() as u8);
    for w in words {
        b.push(w.len() as u8);
        b.extend_from_slice(w);
    }
    b
}

#[test]
fn the_pinned_block_reads_back_as_its_words() {
    let byte = |i: usize| u8::from_str_radix(&PINNED[i..i + 2], 16).unwrap();
    let pinned: Vec<u8> = (0..PINNED.len()).step_by(2).map(byte).collect();
    let said: &[&[u8]] = &[b"--daemon-address=node.example:18081", b"--untrusted-daemon"];
    assert_eq!(pinned, block(said));
    assert_eq!(check(&pinned), Ok(2));
    assert_eq!(words(&pinned).collect::<Vec<_>>(), said);
}

#[test]
fn only_long_options_of_printable_ascii_are_words() {
    for good in [&b"--a"[..], b"--x9-y", b"--k=v", b"--k=a=b", b"--k=~!"] {
        assert_eq!(check(&block(&[good])), Ok(1), "{good:?}");
    }
    let text = "-a|a|--|--=v|--A|---a|--a_b|--k=|--k=a b|--a;b|--a b".split('|').map(str::as_bytes);
    for bad in text.chain([&b"--k=\x7f"[..], b"--k=\xc3\xa9"]) {
        assert_eq!(check(&block(&[bad])), Err(E::Word), "{bad:?}");
    }
}

#[test]
fn a_person_s_word_names_an_option_by_its_whole_name() {
    let listed = b"--daemon-address=node.example:18081";
    assert_eq!(name(listed), b"--daemon-address");
    assert!(names(b"--daemon-address", listed) && names(b"--daemon-address=other:1", listed));
    let others: [&[u8]; 5] = [b"--daemon-addresses", b"--daemon", b"daemon-address", b"--x", b""];
    for other in others {
        assert!(!names(other, listed), "{other:?}");
    }
}
