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

//! Each reason a NØNOS cannot be updated in place fits the column it is
//! shown in, where "holds now" would say it holds NØNOS, and so do the row
//! an update keeps, its label, and its card's caption.

use crate::installer_layout_tests::bodies;
use crate::literals::literals;
use crate::{label_w, measure, Face};

fn caps(s: &str) -> String {
    s.to_ascii_uppercase()
}

#[test]
fn every_reason_and_row_fits_its_column() {
    let all = literals("../nonos_disk/src/keep", false);
    let reasons: Vec<_> = all.into_iter().filter(|l| l.file == "why.rs").collect();
    let kept = "data volume (1000.2 GB), its key, the plan and the table".to_string();
    assert!(reasons.len() >= 8, "the reasons were read");
    for b in bodies() {
        let (m, value) = (&b.m, b.w - 2 * b.m.inset - b.m.label_w);
        for t in reasons.iter().map(|l| &l.text).chain([&kept]) {
            let width = measure(t, Face::Body, m.body_px, 0.0);
            assert!(width <= value, "{}: \"{t}\" is {width}, its column {value}", b.at);
        }
        assert!(label_w(&caps("kept"), m.label_px) + m.unit <= m.label_w, "{}", b.at);
        let caption = label_w(&caps("what is kept and written"), m.label_px);
        assert!(caption + 2 * m.inset <= b.w, "{}", b.at);
    }
}
