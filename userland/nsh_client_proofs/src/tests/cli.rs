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

//! The `nsh-verify-record` tool over real files: whose record it is, a
//! complete record, one cut short, one refused, one holding a provisional
//! `BOX` entry, and what it says it did not check.

use super::cli_kit::{dir, file, hex, tool};
use super::record_box::{box_body, with_box};
use crate::kit::keys;
use crate::kit::record::Rec;

#[test]
fn the_tool_reports_whose_record_and_its_verdict() {
    let d = dir("verdicts");
    let rec = Rec::good(70);
    let (code, lines) = tool(&d, &rec.bytes(), &[]);
    assert_eq!(code, 0);
    let whose = format!(
        "session {} subject {} start_ms 1000",
        hex(&[0x5E; 16]),
        hex(&keys::user().digest())
    );
    assert_eq!(lines[0], whose);
    assert_eq!(lines[1], "record complete: 71 entries, end reason 1");
    assert_eq!(
        lines[2],
        "registry entries not checked offline; the registry root is bound by the quote"
    );
    let ckpt = file(&d, "ckpt", &rec.ckpt(63));
    let (code, lines) = tool(&d, &rec.no_footer(), &["--ckpt", &ckpt]);
    assert_eq!((code, lines[1].as_str()), (3, "cut short at seq 63"));
    let (code, lines) = tool(&d, &rec.no_footer(), &[]);
    assert_eq!(code, 1);
    assert!(lines[0].starts_with("record refused: Parse"));
    let (code, lines) = tool(&d, &with_box(&box_body(1, 1)).bytes(), &[]);
    let boxed = "record complete: 5 entries, end reason 1; holds provisional BOX entries";
    assert_eq!((code, lines[1].as_str()), (4, boxed));
}
