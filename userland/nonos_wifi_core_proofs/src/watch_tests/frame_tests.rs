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

//! The SA Query and probe builders: what they make reads back, and what they
//! cannot carry is refused.

use super::sim::STA;
use nonos_wifi_core::dot11::probe::probe_to;
use nonos_wifi_core::pmf::{parse_sa_query, sa_query_frame, SaQueryAction};

#[test]
fn the_builders_refuse_what_they_cannot_carry() {
    let q = (SaQueryAction::Response, 0xBEEFu16.to_le_bytes());
    assert_eq!(parse_sa_query(&sa_query_frame(STA, STA, 0, q.0, q.1)[24..]), Some(q));
    for bad in [&[8u8, 2, 0, 0][..], &[3, 0, 0, 0], &[8, 0, 0]] {
        assert_eq!(parse_sa_query(bad), None, "{bad:?}");
    }
    assert!(probe_to(STA, STA, &[b'n'; 32], 0).is_some());
    assert!(probe_to(STA, STA, &[b'n'; 33], 0).is_none());
}
