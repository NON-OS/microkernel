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
//! HTTP/1.1 for a client, with no I/O of its own.

//! What address is downloaded, and what is refused.

use nonos_download::parse;

#[test]
fn https_addresses_with_ports_queries_and_none_at_all() {
    let u = parse("https://Example.org/a/b.mp3?x=1&y=2#frag").unwrap();
    assert_eq!((u.host.as_str(), u.port, u.target.as_str()), ("example.org", 443, "/a/b.mp3?x=1&y=2"));
    let u = parse("example.org:8443").unwrap();
    assert_eq!((u.port, u.target.as_str()), (8443, "/"));
    let u = parse("https://example.org?q=1").unwrap();
    assert_eq!(u.target, "/?q=1");
}

#[test]
fn plain_http_and_anything_odd_is_refused_with_a_reason() {
    for bad in [
        "http://example.org/a.mp3",
        "ftp://example.org/a.mp3",
        "https://user:pw@example.org/a.mp3",
        "https://exa mple.org/a.mp3",
        "https://example.org:0/a",
        "https://example.org/a b.mp3",
        "https:///a.mp3",
    ] {
        assert!(parse(bad).is_err(), "{bad}");
    }
    assert!(parse("http://x.org/a").unwrap_err().contains("exit relay"));
}
