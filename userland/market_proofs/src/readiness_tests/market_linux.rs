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

//! install_ready's policy gate, whatever the release's schema (plan/30).

use crate::abi_v3::fixtures::sold;
use crate::chain_hosts::Host;
use crate::install_ready::market::market_gates;

/* plan/30: a linux.* release fetched from an index carries its box, whatever
 * its schema; only a shipped tier (linux.qwen-*, "store:/bin/qwenchat") is
 * boxed by the personality. A "store:" url under any other listing is not. */
#[test]
fn a_schema_2_linux_release_without_its_box_is_not_ready() {
    let mut e = sold().entries[0].clone();
    e.commerce = None;
    e.listing_id = "linux.curl".into();
    let mut rel = e.releases[0].clone();
    rel.ext = None;
    rel.package_url = "https://dl-cdn.alpinelinux.org/curl.apk".into();
    let g = market_gates(&Host, &e, &rel, true);
    assert!(!g.policy && !g.all());
    rel.package_url = "store:/bin/qwenchat".into();
    assert!(!market_gates(&Host, &e, &rel, true).all(), "a store: url is not a tier");
    e.listing_id = "linux.qwen-small".into();
    assert!(market_gates(&Host, &e, &rel, true).all());
}
