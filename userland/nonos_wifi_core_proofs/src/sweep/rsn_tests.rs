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
//! The RSN element: a sound body reads as built, whichever field it ends
//! after; a cut inside a field, a count that runs past the element or a
//! version other than 1 refuses it. The one leniency: one to three octets
//! where the group management cipher would go are left unread.

use super::rng::each;
use super::rsne::build;
use super::rsne_shape::Field;
use nonos_wifi_core::rsn::parse_rsne;

#[test]
fn sound_bodies_read_as_built() {
    each(|rng| {
        let mut s = build(rng);
        assert_eq!(parse_rsne(&s.body), Some(s.want));
        let pmkids = s.fields.iter().any(|f| f.0 == Field::Count { size: 16 });
        let extra = match s.want.group_mgmt {
            Some(_) => rng.below(9),
            None if pmkids => 1 + rng.below(3),
            None => return,
        };
        s.body.extend(rng.bytes(extra));
        assert_eq!(parse_rsne(&s.body), Some(s.want), "{extra} octets past the last field");
    });
}

#[test]
fn a_cut_inside_a_field_refuses_the_element() {
    each(|rng| {
        let mut s = build(rng);
        let (kind, start, end) = s.fields[rng.below(s.fields.len())];
        if end - start < 2 {
            return;
        }
        let cut = start + 1 + rng.below(end - start - 1);
        let read = parse_rsne(&s.body[..cut]);
        s.want.group_mgmt = None;
        assert_eq!(read, (kind == Field::GroupMgmt).then_some(s.want), "cut at {cut}");
    });
}
