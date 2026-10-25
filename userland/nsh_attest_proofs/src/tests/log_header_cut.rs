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

//! The log's header, cut short, empty, or over the event cap.

use nsh_attest::log::replay;
use nsh_attest::LogFault;

use crate::build::tcg::{header, header_with, EV_NO_ACTION};
use crate::build::tcg_event::{digest, ev};

fn fault(log: Vec<u8>) -> Result<(), LogFault> {
    replay(&log).map(|_| ())
}

#[test]
fn an_empty_cut_or_oversize_header_is_refused() {
    assert_eq!(fault(vec![]), Err(LogFault::Short));
    let h = header();
    for cut in 1..h.len() {
        assert_eq!(fault(h[..cut].to_vec()), Err(LogFault::Short), "{cut}");
    }
    let big = header_with(0, EV_NO_ACTION, [0; 20], &vec![0; 65_537]);
    assert_eq!(fault(big), Err(LogFault::EventTooLarge));
    let mut ok = header();
    ok.extend(ev(0, 1, digest(b"x")));
    assert!(fault(ok).is_ok());
}
