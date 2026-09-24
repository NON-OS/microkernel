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
//! Authority cert, continued.

use crate::directory::verify::parse;

/*
 * Served by 49.13.145.234 for v3 identity 88F29CB5FE86A688E31990A3B20BD562D0C089E1
 * on 2026-09-19. A real document rather than a written one, because what is being
 * checked is that the fields are where an authority actually puts them.
 */
const CERT: &[u8] = include_bytes!("../../vectors/authority-cert.txt");

#[test]
fn the_certificate_is_valid_for_about_a_year() {
    let cert = parse(CERT).expect("parses");
    let published = 1_782_575_188u64;
    let span = cert.expires - published;
    assert!(span > 300 * 86_400 && span < 400 * 86_400, "roughly a year, not a day or a decade");
}
#[test]
fn a_certificate_without_an_expiry_is_refused() {
    let mut stripped = alloc::vec::Vec::new();
    for line in CERT.split(|b| *b == b'\n') {
        if line.starts_with(b"dir-key-expires") {
            continue;
        }
        stripped.extend_from_slice(line);
        stripped.push(b'\n');
    }
    assert!(parse(&stripped).is_none(), "no expiry, no certificate");
}
