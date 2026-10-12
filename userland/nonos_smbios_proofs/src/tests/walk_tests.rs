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

//! The walk over well-formed and broken tables, and what is served.

use crate::build::{qemu, structure};
use crate::chassis::chassis_name;
use crate::identity::identity;
use crate::text::text;
use crate::walk::structures;

#[test]
fn qemu_names_its_machine() {
    let t = qemu();
    let s = structures(&t, None);
    assert_eq!(s.iter().map(|x| x.kind).collect::<Vec<_>>(), [0, 1, 3, 127]);
    let id = identity(&s);
    assert_eq!(
        (id.vendor.as_str(), id.product.as_str()),
        ("QEMU", "Standard PC (Q35 + ICH9, 2009)")
    );
    assert_eq!((id.bios_version.as_str(), id.bios_date.as_str()), ("0.0.0", "02/06/2015"));
    assert_eq!((id.chassis, id.board.as_str()), ("Other", ""));
}

#[test]
fn broken_tables_end_the_walk_where_they_break() {
    let good = structure(1, 1, &[1, 0, 0, 0], &["A"]);
    let short_len = [good.clone(), vec![2, 3, 0, 0, 0, 0]].concat();
    assert_eq!(structures(&short_len, None).len(), 1, "length under the header");
    let past_end = [good.clone(), vec![2, 40, 0, 0]].concat();
    assert_eq!(structures(&past_end, None).len(), 1, "formatted area past the table");
    let no_end = [good.clone(), vec![2, 4, 0, 0, b'x', b'y']].concat();
    assert_eq!(structures(&no_end, None).len(), 1, "strings that never end");
    let after_end = [structure(127, 0, &[], &[]), good.clone()].concat();
    assert_eq!(structures(&after_end, None).len(), 1, "nothing after type 127");
    let three = [good.clone(), good.clone(), good].concat();
    assert_eq!(structures(&three, Some(2)).len(), 2, "the entry point's count");
}

#[test]
fn strings_by_index_and_text_made_safe() {
    let t = structure(1, 1, &[2, 0, 9, 0], &["first", "second"]);
    let s = &structures(&t, None)[0];
    assert_eq!(s.string(4), Some(&b"second"[..]));
    assert_eq!(s.string(5), None, "0 names no string");
    assert_eq!(s.string(6), None, "past the strings");
    assert_eq!(s.string(40), None, "past the formatted area");
    assert_eq!(text(b"  Acme\xFFCorp \t"), "Acme?Corp ?");
    assert_eq!(text(&[b'a'; 100]).len(), 64);
    assert_eq!(
        (chassis_name(0x0A), chassis_name(0x89), chassis_name(0x7F)),
        ("Notebook", "Laptop", "Unknown")
    );
}
