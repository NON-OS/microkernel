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
//! The RSN element lied about: a count that runs past it or a version
//! other than 1 refuses it, and garbage inside one RSN body, under an
//! honest length, hides no other element.

use super::check::agree;
use super::model::build as elements;
use super::rng::each;
use super::rsne::build;
use super::rsne_shape::Field;
use nonos_wifi_core::rsn::parse_rsne;

#[test]
fn a_count_past_the_element_or_another_version_refuses_it() {
    each(|rng| {
        let mut s = build(rng);
        let counts: Vec<_> =
            s.fields.iter().filter(|f| matches!(f.0, Field::Count { .. })).collect();
        if let Some(&&(Field::Count { size }, at, end)) = counts.get(rng.below(counts.len().max(1)))
        {
            let n = ((s.body.len() - end) / size + 1 + rng.below(50)).min(0xffff) as u16;
            let mut lie = s.body.clone();
            lie[at..end].copy_from_slice(&n.to_le_bytes());
            assert_eq!(parse_rsne(&lie), None, "a count of {n}");
        }
        let v = (1 + 1 + rng.below(0xfffe)) as u16;
        s.body[..2].copy_from_slice(&v.to_le_bytes());
        assert_eq!(parse_rsne(&s.body), None, "version {v}");
    });
}

#[test]
fn a_lie_inside_an_rsn_body_leaves_every_element_found() {
    each(|rng| {
        let mut b = elements(rng);
        for (id, at) in b.at.clone() {
            let len = usize::from(b.tags[at + 1]);
            if id == 48 {
                let junk = rng.bytes(len);
                b.tags[at + 2..at + 2 + len].copy_from_slice(&junk);
                let _ = parse_rsne(&b.tags[at + 2..at + 2 + len]);
            }
        }
        agree(&b.tags, |id| b.first(id, b.at.len()).map(<[u8]>::to_vec));
    });
}
