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

use nonos_settle_proto::https_target;

#[test]
fn only_a_plain_https_name_and_path_pass() {
    assert_eq!(
        https_target(b"https://pkg.example.org/a/b.bin?x=1"),
        Some(("pkg.example.org", "/a/b.bin?x=1"))
    );
    assert_eq!(https_target(b"https://pkg.example.org"), Some(("pkg.example.org", "/")));
    for bad in [
        &b"http://pkg.example.org/a"[..],
        b"https://pkg.example.org:8443/a",
        b"https://user@pkg.example.org/a",
        b"https://pkg.example.org/a b",
        b"https://pkg.example.org/a#frag",
        b"https:///a",
        b"https://localhost/a",
        b"https://.example.org/a",
        b"https://example.org./a",
        b"HTTPS://pkg.example.org/a",
    ] {
        assert_eq!(https_target(bad), None, "{}", String::from_utf8_lossy(bad));
    }
}
