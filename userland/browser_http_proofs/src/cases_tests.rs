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

//! The 54 synthetic RFC 9110/9112 conformance cases (vectors/cases.txt says
//! what each must give), and framing agreeing with parsing on all of them.

use crate::browser::http::response::{frame_len, is_complete, parse};
use crate::vectors::{read, PAGE};

fn cases() -> Vec<(String, String, bool, Vec<u8>)> {
    let list = String::from_utf8(read("cases.txt")).unwrap();
    let rows = list.lines().map(|l| l.split(' ').map(String::from).collect::<Vec<_>>());
    rows.map(|w| {
        (w[0].clone(), w[1].clone(), w[2] == "complete", read(&format!("cases/{}.raw", w[0])))
    })
    .collect()
}

#[test]
fn every_case_parses_as_the_rfcs_say() {
    for (name, want, complete, raw) in cases() {
        let got = parse(&raw);
        let ok = match (want.as_str(), &got) {
            ("reject", None) => true,
            ("reject", Some(r)) => r.status >= 400,
            ("page" | "partial-page", Some(r)) => r.status == 200 && r.body == PAGE,
            ("prefix10", Some(r)) => r.body == PAGE[..10],
            ("partial", Some(_)) => true,
            ("br", None) => true,
            (w, Some(r)) if w.starts_with("empty:") => {
                w[6..] == r.status.to_string() && r.body.is_empty()
            }
            _ => false,
        };
        assert!(ok, "{name}: expected {want}, got {:?}", got.map(|r| (r.status, r.body.len())));
        if complete {
            assert!(is_complete(&raw), "{name}: complete at its end");
        }
    }
}

#[test]
fn a_framed_response_is_complete_exactly_at_its_frame() {
    for (name, _, _, raw) in cases() {
        let Some(n) = frame_len(&raw) else { continue };
        assert!(n <= raw.len(), "{name}: frame {n} past {}", raw.len());
        assert!(is_complete(&raw[..n]), "{name}: complete at its frame end");
        assert!(!is_complete(&raw[..n - 1]), "{name}: not complete one byte short");
        let coded = name == "ce_br" || name == "ce_zstd";
        assert!(coded || parse(&raw[..n]).is_some(), "{name}: framed but not parsed");
    }
}
