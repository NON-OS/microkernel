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
//! What must hold of the element walkers for any tags, given what the
//! building says each id's first sound element is.

use super::model::IDS;
use super::rng::within;
use nonos_wifi_core::dot11::ies::find_element;
use nonos_wifi_core::dot11::parse::find_ie;

/// Both walkers against the building, for every id the station reads.
pub fn agree(tags: &[u8], first: impl Fn(u8) -> Option<Vec<u8>>) {
    for id in IDS {
        let (whole, body) = (find_element(tags, id), find_ie(tags, id));
        assert_eq!(whole.map(<[u8]>::to_vec), first(id), "element {id}");
        assert_eq!(body, whole.map(|w| &w[2..]), "the two walkers agree on {id}");
        assert!(whole.is_none_or(|w| within(tags, w)));
    }
}
