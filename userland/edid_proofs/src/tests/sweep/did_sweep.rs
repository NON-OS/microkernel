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

//! DisplayID extensions whose section and block lengths, versions, tags
//! and revisions lie: every record of an honest timing block is found.

use super::ext_did::did;
use super::gen::Kind;
use super::look::look;
use super::rng::Rng;
use crate::build::base;
use crate::build_ext::with;

#[test]
fn every_honest_record_is_found_and_no_lie_reaches_a_mode() {
    let mut r = Rng::new(0xD15D_1D00);
    let (mut found, mut lied, mut refused) = (0usize, 0usize, 0usize);
    for _ in 0..30_000 {
        let exts: Vec<_> = (0..r.range(1, 3)).map(|_| did(&mut r)).collect();
        let raw = with(base(), &exts.iter().map(|d| d.bytes).collect::<Vec<_>>());
        let seen = look(&raw).expect("the base block is honest");
        let n = |k: Kind| exts.iter().flat_map(|d| &d.records).filter(|&&s| s == k).count();
        assert!(seen.did >= n(Kind::Honest), "{} of {}", seen.did, n(Kind::Honest));
        found += n(Kind::Honest);
        lied += n(Kind::Lied);
        refused += exts.iter().filter(|d| d.records.is_empty()).count();
    }
    assert!(found >= 5_000 && lied >= 5_000 && refused >= 1_000, "{found} {lied} {refused}");
}
