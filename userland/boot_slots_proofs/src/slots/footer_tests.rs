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

//! The kernel reads its own signed file as the loader read it before the jump:
//! the same regions out of an honest file, and the same refusal of every other.

use super::footer::regions;
use crate::fixture::{signed_file, HAS_ZK_PROOF};
use crate::image_format::parse_image_footer;

pub(super) fn loader(f: &[u8]) -> Option<(&[u8], &[u8])> {
    let p = parse_image_footer(f).ok()?;
    Some((p.kernel_bytes, p.proof_bytes?))
}

pub(super) fn ours(f: &[u8]) -> Option<(&[u8], &[u8])> {
    regions(f).map(|r| (r.kernel, r.proof))
}

#[test]
fn a_signed_file_splits_as_the_loader_splits_it() {
    let f = signed_file(&[0x7F; 4096], &[0xC4; 300], HAS_ZK_PROOF, 2);
    let got = ours(&f).expect("regions");
    assert_eq!((got.0, got.1), (&[0x7F; 4096][..], &[0xC4; 300][..]));
    assert_eq!(Some(got), loader(&f));
}

#[test]
fn a_file_without_a_proof_has_no_regions() {
    for (flags, proof) in [(0u16, &[0xC4u8; 300][..]), (HAS_ZK_PROOF, &[][..])] {
        let f = signed_file(&[0x7F; 512], proof, flags, 1);
        assert!(parse_image_footer(&f).is_ok(), "the loader parses it");
        assert_eq!((ours(&f), loader(&f)), (None, None));
    }
}

#[test]
fn a_cut_or_extended_file_is_refused_by_both() {
    let f = signed_file(&[0x7F; 512], &[0xC4; 100], HAS_ZK_PROOF, 1);
    for cut in [0, 1, 63, 64, f.len() - 1] {
        assert_eq!((ours(&f[..cut]), loader(&f[..cut])), (None, None), "cut at {cut}");
    }
    let mut g = f.clone();
    g.push(0);
    assert_eq!((ours(&g), loader(&g)), (None, None));
}
