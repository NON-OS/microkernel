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

//! Q4: the uri typed for a release is https, at most MAX_URL, and names
//! only a DNS host: no user, password, port or fragment.

use nonos_marketplace_abi::SettleMode;
use nonos_settle_proto::MAX_URL;

use super::fixture::{entry, K, ME};
use super::intents_release::URI;
use crate::intent::{publish_of, Why};

#[test]
fn the_uri_is_https_whole_and_without_a_user() {
    let e = entry(SettleMode::Escrow);
    let uri = |u: &str| publish_of(&K, &e, &ME, 0, u).err();
    let mut long = String::from("https://h.org/");
    long.extend(std::iter::repeat_n('a', MAX_URL - long.len()));
    assert_eq!((uri(URI), uri(&long)), (None, None), "an @ in the path, and MAX_URL, taken");
    long.push('a');
    for bad in [long.as_str(), "http://example.org/r", "https://user@example.org/r"] {
        assert_eq!(uri(bad), Some(Why::Uri), "{bad}");
    }
    for bad in ["https://u:pw@example.org/r", "https://example.org:443/r", "https://example.org/#x"]
    {
        assert_eq!(uri(bad), Some(Why::Uri), "{bad}");
    }
    assert_eq!((uri("https://example/r"), uri("")), (Some(Why::Uri), Some(Why::Uri)));
}
