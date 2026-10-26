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

//! F-120: the capsule's \_S5 line against what the kernel then writes,
//! for \_S5 unusable or each pair to 9/9, tables read or not, and each
//! scanned pair to 9/9. The model, stated apart: the kernel takes the
//! capsule's pair only with its tables read and three-bit values (else
//! EINVAL or ENODEV), and writes its scanned pair if that fits, else the
//! one it took, else none. "handed" is said exactly when it took the pair.

#[path = "shutdown_kit.rs"]
mod kit;
#[path = "../../../src/hardware/broker/acpi/s5_choice.rs"]
mod s5_choice;

use kit::say::s5_line;
use kit::{pairs, sleep_request, text};

#[test]
fn the_line_says_the_path_the_write_takes() {
    let (mut via_capsule, mut via_scan) = (0, 0);
    for types in pairs() {
        for tables in [false, true] {
            let handed =
                types.map(|(a, b)| match (sleep_request::valid(5, a.into(), b.into()), tables) {
                    (false, _) => Err(22),
                    (true, false) => Err(19),
                    (true, true) => Ok(()),
                });
            let line = text(|o| s5_line(types, handed.unwrap_or(Ok(())), o));
            let taken = types.filter(|(a, b)| tables && *a <= 7 && *b <= 7);
            let stands = "; the kernel's scanned \\_S5, if any, stands";
            let want = match (types, taken, handed) {
                (None, ..) => format!("\\_S5 unusable at shutdown{stands}"),
                (Some((a, b)), Some(_), _) => {
                    format!("\\_S5 {a}/{b} handed to the kernel's soft-off")
                }
                (Some((a, b)), None, Some(Err(e))) => {
                    format!("\\_S5 {a}/{b} refused by the kernel, errno {e}{stands}")
                }
                _ => unreachable!(),
            };
            assert_eq!(line, want);
            for scan in pairs() {
                let scan = scan.filter(|_| tables);
                let wrote = s5_choice::choose(scan, taken);
                let fits = |p: &(u8, u8)| p.0 <= 7 && p.1 <= 7;
                assert_eq!(wrote.map(|w| w.0), scan.filter(fits).or(taken), "{types:?} {scan:?}");
                let said = s5_choice::posture(wrote.map(|w| w.1));
                assert_eq!(said.ends_with(b"=none"), wrote.is_none(), "the posture says none");
                if let Some((_, s5_choice::Source::Interpreter)) = wrote {
                    assert!(line.contains(" handed "), "the capsule's pair written: it said so");
                    via_capsule += 1;
                }
                via_scan += usize::from(matches!(wrote, Some((_, s5_choice::Source::Scan))));
                assert_eq!(line.contains(" handed "), !line.contains("stands"));
            }
        }
    }
    assert!(via_capsule > 1000 && via_scan > 5000, "{via_capsule} {via_scan}");
}
