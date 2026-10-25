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

//! The rule case by case: probes to a silent server, an answer in time, and
//! a full table, and settle wiring it beside the late line.

use super::silence::{Pair, Silence, SLOTS};

const W: u64 = 5000;
const SETTLE: &str = include_str!("../../../../src/syscall/microkernel/ipc/call/settle.rs");

fn p(caller: u32) -> Pair {
    Pair { caller, port: 4444 }
}

#[test]
fn probes_to_a_silent_server_report_once_the_window_is_spanned() {
    let mut s = Silence::new(W);
    let mut reports = Vec::new();
    for i in 0..30u64 {
        let start = i * 500;
        if let [Some(x), None] = s.late(p(7), start, start + 20) {
            reports.push((x, start + 20));
        }
    }
    assert_eq!(reports, [(p(7), 5020), (p(7), 10020)], "at 5 s, then 5 s later");
}

#[test]
fn an_answer_in_time_ends_the_silence() {
    let mut s = Silence::new(W);
    assert_eq!(s.late(p(7), 0, 100), [None, None]);
    s.answered(p(7));
    assert_eq!(s.late(p(7), 4000, 5100), [None, None], "the silence began again");
    assert_eq!(s.late(p(7), 9000, 9100), [Some(p(7)), None]);
}

#[test]
fn a_full_table_evicts_the_longest_silent_and_reports_it() {
    let mut s = Silence::new(W);
    for c in 0..SLOTS as u32 {
        assert_eq!(s.late(p(100 + c), u64::from(c), u64::from(c) + 1), [None, None]);
    }
    assert_eq!(s.late(p(1), 50, 60), [None, Some(p(100))], "the oldest goes, reported");
    assert_eq!(s.late(p(2), 51, 61), [None, Some(p(101))]);
}

#[test]
fn settle_answers_on_a_reply_and_a_late_call_alone_says_nothing() {
    let code: String = SETTLE.split_whitespace().collect();
    assert!(code.contains("ifrc>=0{silence_table::answered(pair);}"));
    let late = "NoAnswer::Late=>silence_table::report(&STARVED,pair,&endpoint.name,waited.1),";
    assert!(code.contains(late), "the silence is asked on every late call");
    assert!(!code.contains(".late("), "no late line of its own");
}
