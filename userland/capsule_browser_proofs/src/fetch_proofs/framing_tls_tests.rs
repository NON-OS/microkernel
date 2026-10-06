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

//! Over TLS, a framed response ends the same way, read record by record.

use super::fetch_tls_fixtures::{server_records, settled};
use super::fetch_wire::FakeWire;
use super::framing_tests::{ends_at_last_byte, reading, CHUNKED, SIZED};
use crate::browser::fetch::run::run;
use crate::browser::fetch::tls::decrypt;
use crate::browser::fetch::types::Phase;

#[test]
fn over_tls_both_end_the_same_way() {
    ends_at_last_byte(&server_records(SIZED, 16, 0), "https", Phase::Decrypt);
    let wire = server_records(CHUNKED, 7, 0);
    ends_at_last_byte(&wire, "https", Phase::Decrypt);
    let mut w = FakeWire::at(0);
    let mut f = reading("https");
    f.tls = Some(settled());
    w.deliver(11, &wire);
    run(&mut w, &mut f, 30);
    assert_eq!(decrypt(&mut f).as_deref(), Some(CHUNKED));
}
