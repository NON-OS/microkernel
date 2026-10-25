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

//! A seeded sweep: random log lines with MACs, addresses, UUIDs, keys and
//! serials planted anywhere in them, glued to words or not. Every report the
//! writer makes from them parses, and no planted value is found in one.

use nonos_hwreport::parse::check;
use nonos_hwreport::report::Report;

use crate::rng::Rng;
use crate::sweep_gen::line;

#[test]
fn no_planted_value_survives_into_a_report_and_every_report_parses() {
    let mut rng = Rng::new(0x0b5e55ed);
    for round in 0..20_000 {
        let mut planted = Vec::new();
        let lines: Vec<String> =
            (0..1 + rng.below(12)).map(|_| line(&mut rng, &mut planted)).collect();
        let mut buf = vec![0u8; 4096];
        let mut r = Report::new(&mut buf, b"1536ccef3", b"standard-dev", false);
        for l in &lines {
            r.log(l.as_bytes());
        }
        let (len, _) = r.finish();
        let text = String::from_utf8(buf[..len].to_vec()).unwrap();
        check(text.as_bytes())
            .unwrap_or_else(|e| panic!("round {round}: {e:?}\n{text}\n{lines:?}"));
        for s in &planted {
            assert!(!text.contains(s.as_str()), "round {round}: {s} in\n{text}");
        }
    }
}
