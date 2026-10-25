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

//! The parser over the writer's own reports, every way a line can be wrong,
//! random bytes, and every buffer size.

use crate::sample::sample;
use nonos_hwreport::parse::{check, Fault};

#[test]
fn the_writers_report_parses_with_every_line_counted() {
    let mut buf = [0u8; 2048];
    let (len, cut) = sample(&mut buf);
    assert!(!cut);
    let s = check(&buf[..len]).unwrap();
    assert_eq!((s.pci, s.acpi, s.log, s.dropped, s.unconfined, s.cut), (1, 1, 1, 1, true, false));
}

#[test]
fn each_wrong_line_is_refused_by_its_number() {
    let mut buf = [0u8; 2048];
    let (len, _) = sample(&mut buf);
    let good = String::from_utf8(buf[..len].to_vec()).unwrap();
    let cases = [
        (good.replace("NONOS-HWREPORT 1", "NONOS-HWREPORT 2"), Fault::NoHeader),
        (good.replace("posture dma-unconfined", "posture maybe"), Fault::Field(3)),
        (good.replace("claimed=1", "claimed=2"), Fault::Field(4)),
        (good.replace("1536ccef3", "15 36"), Fault::Field(2)),
        (good.replace("end 7", "end 8"), Fault::Count(8)),
        (good.replace("end 7", "end 07"), Fault::Field(8)),
        (good.replace("reason=", "reason=10.0.2.15 "), Fault::Leak(6)),
        (good.replace("posture=dma", "serial 4 posture=dma"), Fault::Leak(6)),
        (good.trim_end().to_string(), Fault::NoEnd),
        (format!("{good}log x\n"), Fault::AfterEnd(9)),
        (good.replace(&"11".repeat(32), &"11".repeat(31)), Fault::Field(5)),
    ];
    for (text, want) in cases {
        assert_eq!(check(text.as_bytes()), Err(want), "{text}");
    }
}
