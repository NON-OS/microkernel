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

use nonos_market_proto::parse_commerce;

use super::commerce_encode::encode;
use super::fixtures::sold;

#[test]
fn a_cut_or_padded_reply_is_refused() {
    let entry = sold().entries[0].clone();
    let body = encode(&entry, &entry.releases[0], true).unwrap();
    for n in 0..body.len() {
        assert!(parse_commerce(&body[..n]).is_none(), "{n} bytes");
    }
    let mut longer = body.clone();
    longer.push(0);
    assert!(parse_commerce(&longer).is_none());
}
