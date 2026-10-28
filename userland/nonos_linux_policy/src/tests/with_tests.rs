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

//! An extra-package block is read in one spelling, and a tail splits where
//! its listed arguments end, so a listing with none reads as it always has.

use super::args_tests::block as args_block;
use crate::args;
use crate::with::{check, names, split, WithError as E, MAX_NAMES};

/// The catalogue's listed_with.py writes these same bytes for these names.
const PINNED: &str =
    "4e4c58574954483102117a6174687572612d7064662d6d757064660c706f70706c65722d64617461";

fn block(names: &[&[u8]]) -> Vec<u8> {
    let mut b = b"NLXWITH1".to_vec();
    b.push(names.len() as u8);
    names.iter().for_each(|n| b.extend([&[n.len() as u8][..], n].concat()));
    b
}

#[test]
fn the_pinned_block_reads_back_as_its_names() {
    let byte = |i: usize| u8::from_str_radix(&PINNED[i..i + 2], 16).unwrap();
    let pinned: Vec<u8> = (0..PINNED.len()).step_by(2).map(byte).collect();
    let said: &[&[u8]] = &[b"zathura-pdf-mupdf", b"poppler-data"];
    assert_eq!((&pinned, check(&pinned)), (&block(said), Ok(2)));
    assert_eq!(names(&pinned).collect::<Vec<_>>(), said);
}

#[test]
fn only_package_names_up_to_the_count_once_each_are_read() {
    let long = [b'a'; 64];
    for good in [&b"a"[..], b"0ad", b"py3-a.b_c+d", b"a-", &long] {
        assert_eq!(check(&block(&[good])), Ok(1), "{good:?}");
    }
    for bad in [&b""[..], b"A", b"-a", b".a", b"_a", b"a/b", b"a b", b"\xc3\xa9", &[b'a'; 65]] {
        assert_eq!(check(&block(&[bad])), Err(E::Name), "{bad:?}");
    }
    let nine: Vec<[u8; 1]> = (b'a'..).take(MAX_NAMES + 1).map(|c| [c]).collect();
    let nine: Vec<&[u8]> = nine.iter().map(|n| &n[..]).collect();
    assert_eq!(check(&block(&nine[..MAX_NAMES])), Ok(MAX_NAMES));
    assert_eq!(check(&block(&nine)), Err(E::Count));
    assert_eq!(check(&block(&[])), Err(E::Count));
    assert_eq!(check(b"NLXWITH2\x01\x01a"), Err(E::Count));
    assert_eq!(check(b"NLXWITH1\x02\x01a"), Err(E::Name));
    assert_eq!(check(&block(&[b"a", b"a"])), Err(E::Trailing));
    assert_eq!(check(b"NLXWITH1\x01\x02a"), Err(E::Name));
    assert_eq!(check(b"NLXWITH1\x01\x01a\x00"), Err(E::Trailing));
}

#[test]
fn a_tail_splits_where_its_arguments_end() {
    let (args, with) = (args_block(&[b"--off", b"--k=v"]), block(&[b"y"]));
    assert_eq!(split(&[&args[..], &with].concat()), (&args[..], &with[..]));
    assert_eq!(split(&args), (&args[..], &[][..]));
    assert_eq!(split(&with), (&[][..], &with[..]));
    assert_eq!(split(b""), (&[][..], &[][..]));
    assert_eq!(split(args::MAGIC), (&args::MAGIC[..], &[][..]));
    let short = &args[..args.len() - 1];
    assert_eq!(split(short), (short, &[][..]), "a short block is all arguments, and fails them");
}
