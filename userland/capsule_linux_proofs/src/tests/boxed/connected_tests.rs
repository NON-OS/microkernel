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

//! The destinations a run connected to: each once, in the order first
//! reached, and never more than a destination list can hold.

use super::net_report_tests::reached;
use crate::boxed::connected::Connected;

#[test]
fn each_destination_reached_is_named_once_in_first_order() {
    let c = reached(&[b"api.example.com:443", b"10.0.2.2:80", b"api.example.com:443"]);
    assert_eq!(c.all(), [&b"api.example.com:443"[..], b"10.0.2.2:80"]);
    let many: Vec<Vec<u8>> = (0..100).map(|i| format!("h:{i}").into_bytes()).collect();
    let mut full = Connected::default();
    many.iter().for_each(|d| full.note(d));
    assert_eq!(full.all().len(), 64, "never more than a list can hold");
}
