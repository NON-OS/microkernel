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

//! Listings and a typed search, as the store's tests build them.

use nonos_market_proto::App;

use crate::store::listing::Listing;
use crate::store::search::Search;

pub fn typed(text: &[u8]) -> Search {
    let mut s = Search::default();
    for b in text {
        assert!(s.push(*b));
    }
    s
}

pub fn listing(id: &[u8], name: &[u8]) -> Listing {
    Listing::new(id.to_vec(), [0; 32], name.to_vec(), true)
}

pub fn described(mut l: Listing, description: &[u8]) -> Listing {
    let detail = App {
        name: l.name.clone(),
        publisher: b"p".to_vec(),
        description: description.to_vec(),
        releases: 1,
    };
    l.known.detail = Some(detail);
    l.known.described = true;
    l
}
